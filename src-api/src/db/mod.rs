//! Database persistence layer for Code Warrior API
//!
//! Uses sqlx with PostgreSQL (Neon) for session and progress storage.

pub mod models;
pub mod operations;
pub mod schema;

// Re-export only what's used by main.rs (auth handlers use operations directly)
pub use operations::{
    delete_save_slot_for_user, get_save_slot_by_user_id, get_session_by_user_id,
    list_save_slots_by_user_id, update_user_session_state, upsert_save_slot_for_user,
    upsert_session_by_user_id,
};

use sqlx::{Pool, Postgres};

/// Type alias for the database connection pool
pub type DbPool = Pool<Postgres>;

/// Initialize database tables (run migrations)
pub async fn init_database(pool: &DbPool) -> Result<(), sqlx::Error> {
    schema::run_migrations(pool).await
}
