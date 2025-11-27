//! Database persistence layer for Code Warrior API
//!
//! Uses sqlx with PostgreSQL (Neon) for session and progress storage.

pub mod models;
pub mod operations;
pub mod schema;

pub use models::*;
pub use operations::*;

use sqlx::{Pool, Postgres};

/// Type alias for the database connection pool
pub type DbPool = Pool<Postgres>;

/// Create a new database connection pool
pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}

/// Initialize database tables (run migrations)
pub async fn init_database(pool: &DbPool) -> Result<(), sqlx::Error> {
    schema::run_migrations(pool).await
}
