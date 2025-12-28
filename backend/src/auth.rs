//! Authentication module with JWT-based login/registration

use crate::{AppError, AppState};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::{FromRef, State},
    http::header,
    Json,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: i64,
    pub iat: i64,
}

/// Authentication state extracted from JWT
#[derive(Debug, Clone)]
pub struct AuthState {
    pub user_id: Uuid,
    pub email: String,
}

/// Registration request payload
#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

/// Login request payload
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

/// Authentication response with token
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

/// User response (safe to return to client)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

/// Register a new user
pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    if state.db.get_user_by_email(&payload.email).await?.is_some() {
        return Err(AppError::Conflict("Email already registered".to_string()));
    }

    // Hash password
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(e.to_string()))?
        .to_string();

    let user_id = Uuid::new_v4();
    let now = Utc::now();

    state
        .db
        .create_user(user_id, &payload.name, &payload.email, &password_hash, now)
        .await?;

    let token = create_jwt(&state.jwt_secret, user_id, &payload.email)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user_id.to_string(),
            name: payload.name,
            email: payload.email,
            created_at: now.to_rfc3339(),
        },
    }))
}

/// Login an existing user
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let user = state
        .db
        .get_user_by_email(&payload.email)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|e| AppError::Internal(e.to_string()))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let token = create_jwt(&state.jwt_secret, user.id, &user.email)?;

    Ok(Json(AuthResponse {
        token,
        user: UserResponse {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            created_at: user.created_at.to_rfc3339(),
        },
    }))
}

/// Get current authenticated user
pub async fn me(
    State(state): State<AppState>,
    auth: AuthState,
) -> Result<Json<UserResponse>, AppError> {
    let user = state
        .db
        .get_user_by_id(auth.user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(UserResponse {
        id: user.id.to_string(),
        name: user.name,
        email: user.email,
        created_at: user.created_at.to_rfc3339(),
    }))
}

fn create_jwt(secret: &str, user_id: Uuid, email: &str) -> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::days(7);

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: exp.timestamp(),
        iat: now.timestamp(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub fn verify_jwt(secret: &str, token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))
}

/// Axum extractor for authenticated requests (Rust 1.75+ native async traits)
impl<S> axum::extract::FromRequestParts<S> for AuthState
where
    AppState: axum::extract::FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Invalid authorization format".to_string()))?;

        let claims = verify_jwt(&app_state.jwt_secret, token)?;

        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))?;

        Ok(AuthState {
            user_id,
            email: claims.email,
        })
    }
}
