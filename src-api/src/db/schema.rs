//! Database schema definitions and migrations

use sqlx::{Pool, Postgres};

/// Run database migrations to create tables
pub async fn run_migrations(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Create users table (auth system)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            username VARCHAR(50) UNIQUE,
            password_hash VARCHAR(255),
            email_verified BOOLEAN DEFAULT FALSE,
            is_suspended BOOLEAN DEFAULT FALSE,
            is_blacklisted BOOLEAN DEFAULT FALSE,
            total_xp INTEGER DEFAULT 0,
            last_login_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create index on email for faster lookups
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_users_email ON users(email)
        "#,
    )
    .execute(pool)
    .await?;

    // Create email_tokens table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS email_tokens (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            token_type VARCHAR(50) NOT NULL,
            token_hash VARCHAR(255) NOT NULL,
            expires_at TIMESTAMPTZ NOT NULL,
            used_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_email_tokens_hash ON email_tokens(token_hash)
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_email_tokens_user_id ON email_tokens(user_id)
        "#,
    )
    .execute(pool)
    .await?;

    // Add used_at column to email_tokens if it doesn't exist
    sqlx::query(
        r#"
        ALTER TABLE email_tokens ADD COLUMN IF NOT EXISTS used_at TIMESTAMPTZ
        "#,
    )
    .execute(pool)
    .await?;

    // Create oauth_connections table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS oauth_connections (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            provider VARCHAR(50) NOT NULL,
            provider_user_id VARCHAR(255) NOT NULL,
            provider_email VARCHAR(255),
            access_token TEXT,
            refresh_token TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            UNIQUE(provider, provider_user_id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_oauth_connections_user_id ON oauth_connections(user_id)
        "#,
    )
    .execute(pool)
    .await?;

    // Create sessions table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            device_id VARCHAR(255) UNIQUE NOT NULL,
            user_id UUID REFERENCES users(id),
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
            user_id UUID REFERENCES users(id),
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
            user_id UUID REFERENCES users(id),
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

    // Add username index
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_users_username ON users(username) WHERE username IS NOT NULL
        "#,
    )
    .execute(pool)
    .await?;

    // Add user_id indexes for existing tables
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id) WHERE user_id IS NOT NULL
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_player_progress_user_id ON player_progress(user_id) WHERE user_id IS NOT NULL
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_save_slots_user_id ON save_slots(user_id) WHERE user_id IS NOT NULL
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
