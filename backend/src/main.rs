//! Backend server with Axum 0.8.7
//! Provides authentication API endpoints and serves Leptos frontend

mod auth;
mod db;
mod error;

use axum::{
    routing::{get, post},
    Router,
};
use frontend::{shell, App};
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use auth::{AuthState, Claims};
pub use db::Database;
pub use error::AppError;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize database
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:./data.db?mode=rwc".to_string());
    let db = Database::new(&database_url).await?;
    db.run_migrations().await?;

    // Create app state
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-super-secret-jwt-key-change-in-production".to_string());

    let state = AppState {
        db: Arc::new(db),
        jwt_secret,
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Leptos Configuration
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // API Router (Authentication & Logic)
    let api_router = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/register", post(auth::register))
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/me", get(auth::me))
        .layer(cors)
        .with_state(state);

    // Main App Router (Leptos + API)
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .merge(api_router)
        .nest_service(
            "/pkg",
            tower_http::services::ServeDir::new("target/site/pkg"),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(leptos_options);

    // Start server
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("🚀 Server running on http://{}", &addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}
