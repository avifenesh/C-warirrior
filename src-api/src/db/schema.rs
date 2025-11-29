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

    // Create save_slots table for Save/Load feature
    // First drop old table if schema changed (no device_id column)
    sqlx::query(
        r#"
        DO $$
        BEGIN
            IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'save_slots')
               AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'save_slots' AND column_name = 'device_id')
            THEN
                DROP TABLE save_slots;
            END IF;
        END $$;
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS save_slots (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id VARCHAR(255) NOT NULL,
            slot_name VARCHAR(100) NOT NULL,
            save_data JSONB NOT NULL,
            total_xp INTEGER DEFAULT 0,
            levels_completed INTEGER DEFAULT 0,
            current_level VARCHAR(50),
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(device_id, slot_name)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_save_slots_device_id ON save_slots(device_id)
        "#,
    )
    .execute(pool)
    .await?;

    // Create quest_progress table for multi-quest tracking
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS quest_progress (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id VARCHAR(255) NOT NULL,
            level_id VARCHAR(50) NOT NULL,
            quest_id VARCHAR(100) NOT NULL,
            xp_earned INTEGER DEFAULT 0,
            completed_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(device_id, quest_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_quest_progress_device_id ON quest_progress(device_id)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_quest_progress_level_id ON quest_progress(device_id, level_id)
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
