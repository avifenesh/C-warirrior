//! Database schema definitions and migrations

use sqlx::{Pool, Postgres};

/// Run database migrations to create tables
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Create sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id VARCHAR(255) UNIQUE NOT NULL,
            game_state JSONB NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create player_progress table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS player_progress (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id VARCHAR(255) UNIQUE NOT NULL,
            completed_levels TEXT[] DEFAULT '{}',
            total_xp INTEGER DEFAULT 0,
            current_level VARCHAR(50),
            achievements TEXT[] DEFAULT '{}',
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on device_id for faster lookups
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_sessions_device_id ON sessions(device_id)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_player_progress_device_id ON player_progress(device_id)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
