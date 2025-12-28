//! Database module with SQLite support

use chrono::{DateTime, Utc};
use sqlx::{FromRow, SqlitePool, sqlite::SqlitePoolOptions};
use uuid::Uuid;

/// User model
#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database wrapper
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Create a new database connection pool
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY NOT NULL,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Create index on email for faster lookups
        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)
            "#,
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("Database migrations completed");
        Ok(())
    }

    /// Create a new user
    pub async fn create_user(
        &self,
        id: Uuid,
        name: &str,
        email: &str,
        password_hash: &str,
        created_at: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO users (id, name, email, password_hash, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .bind(created_at.to_rfc3339())
        .bind(created_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, password_hash, created_at, updated_at
            FROM users
            WHERE email = ?
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|r| r.into()))
    }

    /// Get user by ID
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT id, name, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.map(|r| r.into()))
    }
}

/// Internal row type for SQLite (uses TEXT for UUID and DateTime)
#[derive(Debug, FromRow)]
struct UserRow {
    id: String,
    name: String,
    email: String,
    password_hash: String,
    created_at: String,
    updated_at: String,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: Uuid::parse_str(&row.id).unwrap_or_default(),
            name: row.name,
            email: row.email,
            password_hash: row.password_hash,
            created_at: DateTime::parse_from_rfc3339(&row.created_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_default(),
            updated_at: DateTime::parse_from_rfc3339(&row.updated_at)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_default(),
        }
    }
}
