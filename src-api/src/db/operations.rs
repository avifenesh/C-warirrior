//! Database CRUD operations for sessions, player progress, and authentication

use super::models::{
    EmailToken, NewEmailToken, NewOAuthConnection, NewOAuthUser, NewProgress, NewSession,
    NewUser, OAuthConnection, PlayerProgress, SaveSlot, Session, User,
};
use super::DbPool;

use rand::Rng;
use serde_json::Value;
use std::time::Duration;
use uuid::Uuid;

/// Simple exponential backoff with jitter for Neon optimization
fn backoff_delay(attempt: u32) -> Duration {
    let base_delay = Duration::from_millis(50);
    let max_delay = Duration::from_millis(1000);
    let jitter_factor = 0.5;

    let exponential_delay = base_delay * 2_u32.pow(attempt.min(8));
    let capped_delay = exponential_delay.min(max_delay);

    let jitter_range = capped_delay.as_secs_f64() * jitter_factor;
    let jitter = rand::thread_rng().gen_range(-jitter_range..=jitter_range);

    Duration::from_secs_f64((capped_delay.as_secs_f64() + jitter).max(0.0))
}

/// Get a session by device ID (with simple retry logic)
pub async fn get_session(pool: &DbPool, device_id: &str) -> Result<Option<Session>, sqlx::Error> {
    let max_attempts = 5;
    let mut attempts = 0;

    loop {
        match sqlx::query_as::<_, Session>(
            r#"
            SELECT id, device_id, game_state, created_at, updated_at
            FROM sessions
            WHERE device_id = $1
            "#,
        )
        .bind(device_id)
        .fetch_optional(pool)
        .await
        {
            Ok(session) => return Ok(session),
            Err(sqlx::Error::PoolTimedOut) | Err(sqlx::Error::Database(_)) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(sqlx::Error::PoolTimedOut);
                }
                let delay = backoff_delay(attempts - 1);
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Create or update a session (upsert) with retry logic
pub async fn save_session(pool: &DbPool, session: &NewSession) -> Result<Session, sqlx::Error> {
    let max_attempts = 5;
    let mut attempts = 0;

    loop {
        match sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (device_id, game_state)
            VALUES ($1, $2)
            ON CONFLICT (device_id)
            DO UPDATE SET game_state = $2, updated_at = NOW()
            RETURNING id, device_id, game_state, created_at, updated_at
            "#,
        )
        .bind(&session.device_id)
        .bind(&session.game_state)
        .fetch_one(pool)
        .await
        {
            Ok(session) => return Ok(session),
            Err(sqlx::Error::PoolTimedOut) | Err(sqlx::Error::Database(_)) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(sqlx::Error::PoolTimedOut);
                }
                let delay = backoff_delay(attempts - 1);
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Update only the game state for a session (with retry logic)
pub async fn update_session_state(
    pool: &DbPool,
    device_id: &str,
    state: &Value,
) -> Result<(), sqlx::Error> {
    let max_attempts = 5;
    let mut attempts = 0;

    loop {
        match sqlx::query(
            r#"
            UPDATE sessions
            SET game_state = $2, updated_at = NOW()
            WHERE device_id = $1
            "#,
        )
        .bind(device_id)
        .bind(state)
        .execute(pool)
        .await
        {
            Ok(_) => return Ok(()),
            Err(sqlx::Error::PoolTimedOut) | Err(sqlx::Error::Database(_)) => {
                attempts += 1;
                if attempts >= max_attempts {
                    return Err(sqlx::Error::PoolTimedOut);
                }
                let delay = backoff_delay(attempts - 1);
                tokio::time::sleep(delay).await;
            }
            Err(e) => return Err(e),
        }
    }
}

/// Create or update player progress (upsert)
pub async fn save_progress(
    pool: &DbPool,
    progress: &NewProgress,
) -> Result<PlayerProgress, sqlx::Error> {
    sqlx::query_as::<_, PlayerProgress>(
        r#"
        INSERT INTO player_progress (device_id, completed_levels, total_xp, current_level, achievements)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (device_id)
        DO UPDATE SET
            completed_levels = $2,
            total_xp = $3,
            current_level = $4,
            achievements = $5,
            updated_at = NOW()
        RETURNING id, device_id, completed_levels, total_xp, current_level, achievements, created_at, updated_at
        "#,
    )
    .bind(&progress.device_id)
    .bind(&progress.completed_levels)
    .bind(progress.total_xp)
    .bind(&progress.current_level)
    .bind(&progress.achievements)
    .fetch_one(pool)
    .await
}

/// Mark a level as completed and add XP in player_progress
pub async fn complete_level(
    pool: &DbPool,
    device_id: &str,
    level_id: &str,
    xp_earned: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO player_progress (device_id, completed_levels, total_xp, current_level, achievements)
        VALUES ($1, ARRAY[$2], $3, NULL, ARRAY[]::text[])
        ON CONFLICT (device_id)
        DO UPDATE SET
            completed_levels = CASE
                WHEN NOT ($2 = ANY(player_progress.completed_levels))
                    THEN array_append(player_progress.completed_levels, $2)
                ELSE player_progress.completed_levels
            END,
            total_xp = player_progress.total_xp + $3,
            updated_at = NOW()
        "#,
    )
    .bind(device_id)
    .bind(level_id)
    .bind(xp_earned)
    .execute(pool)
    .await?;

    Ok(())
}

// Save slot operations for Save/Load feature

/// List all save slots for a device
pub async fn list_save_slots(pool: &DbPool, device_id: &str) -> Result<Vec<SaveSlot>, sqlx::Error> {
    sqlx::query_as::<_, SaveSlot>(
        r#"
        SELECT id, device_id, slot_name, save_data, total_xp, levels_completed, current_level, created_at, updated_at
        FROM save_slots
        WHERE device_id = $1
        ORDER BY updated_at DESC
        "#,
    )
    .bind(device_id)
    .fetch_all(pool)
    .await
}

/// Get a specific save slot
pub async fn get_save_slot(
    pool: &DbPool,
    device_id: &str,
    slot_name: &str,
) -> Result<Option<SaveSlot>, sqlx::Error> {
    sqlx::query_as::<_, SaveSlot>(
        r#"
        SELECT id, device_id, slot_name, save_data, total_xp, levels_completed, current_level, created_at, updated_at
        FROM save_slots
        WHERE device_id = $1 AND slot_name = $2
        "#,
    )
    .bind(device_id)
    .bind(slot_name)
    .fetch_optional(pool)
    .await
}

/// Create or update a save slot
pub async fn upsert_save_slot(
    pool: &DbPool,
    device_id: &str,
    slot_name: &str,
    save_data: &Value,
    total_xp: i32,
    levels_completed: i32,
    current_level: Option<&str>,
) -> Result<SaveSlot, sqlx::Error> {
    sqlx::query_as::<_, SaveSlot>(
        r#"
        INSERT INTO save_slots (device_id, slot_name, save_data, total_xp, levels_completed, current_level)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (device_id, slot_name)
        DO UPDATE SET
            save_data = $3,
            total_xp = $4,
            levels_completed = $5,
            current_level = $6,
            updated_at = NOW()
        RETURNING id, device_id, slot_name, save_data, total_xp, levels_completed, current_level, created_at, updated_at
        "#,
    )
    .bind(device_id)
    .bind(slot_name)
    .bind(save_data)
    .bind(total_xp)
    .bind(levels_completed)
    .bind(current_level)
    .fetch_one(pool)
    .await
}

/// Delete a save slot
pub async fn delete_save_slot(
    pool: &DbPool,
    device_id: &str,
    slot_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM save_slots
        WHERE device_id = $1 AND slot_name = $2
        "#,
    )
    .bind(device_id)
    .bind(slot_name)
    .execute(pool)
    .await?;

    Ok(())
}

/// Create a new user with password
pub async fn create_user(pool: &DbPool, user: &NewUser) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, password_hash, email_verified, is_suspended, is_blacklisted, total_xp, last_login_at, created_at
        "#,
    )
    .bind(&user.email)
    .bind(&user.username)
    .bind(&user.password_hash)
    .fetch_one(pool)
    .await
}

/// Create a new user via OAuth (no password)
pub async fn create_oauth_user(pool: &DbPool, user: &NewOAuthUser) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, username, email_verified)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, password_hash, email_verified, is_suspended, is_blacklisted, total_xp, last_login_at, created_at
        "#,
    )
    .bind(&user.email)
    .bind(&user.username)
    .bind(user.email_verified)
    .fetch_one(pool)
    .await
}

/// Get user by ID
pub async fn get_user_by_id(pool: &DbPool, user_id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, email_verified, is_suspended, is_blacklisted, total_xp, last_login_at, created_at
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

/// Get user by email
pub async fn get_user_by_email(pool: &DbPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, email_verified, is_suspended, is_blacklisted, total_xp, last_login_at, created_at
        FROM users
        WHERE LOWER(email) = LOWER($1)
        "#,
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

/// Get user by username
pub async fn get_user_by_username(pool: &DbPool, username: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, username, password_hash, email_verified, is_suspended, is_blacklisted, total_xp, last_login_at, created_at
        FROM users
        WHERE LOWER(username) = LOWER($1)
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await
}

/// Update user's last login timestamp
pub async fn update_last_login(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET last_login_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Mark user's email as verified
pub async fn verify_user_email(pool: &DbPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET email_verified = TRUE
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Update user's password hash
pub async fn update_user_password(pool: &DbPool, user_id: Uuid, password_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET password_hash = $2
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(pool)
    .await?;
    Ok(())
}

/// Update user's XP (called when completing levels/quests)
pub async fn update_user_xp(pool: &DbPool, user_id: Uuid, xp_delta: i32) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as(
        r#"
        UPDATE users
        SET total_xp = total_xp + $2
        WHERE id = $1
        RETURNING total_xp
        "#,
    )
    .bind(user_id)
    .bind(xp_delta)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Suspend a user
pub async fn suspend_user(pool: &DbPool, user_id: Uuid, suspend: bool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET is_suspended = $2
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(suspend)
    .execute(pool)
    .await?;
    Ok(())
}

/// Blacklist a user
pub async fn blacklist_user(pool: &DbPool, user_id: Uuid, blacklist: bool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET is_blacklisted = $2
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(blacklist)
    .execute(pool)
    .await?;
    Ok(())
}

/// Create a new OAuth connection
pub async fn create_oauth_connection(pool: &DbPool, conn: &NewOAuthConnection) -> Result<OAuthConnection, sqlx::Error> {
    sqlx::query_as::<_, OAuthConnection>(
        r#"
        INSERT INTO oauth_connections (user_id, provider, provider_user_id, provider_email, access_token, refresh_token)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, provider, provider_user_id, provider_email, access_token, refresh_token, created_at, updated_at
        "#,
    )
    .bind(conn.user_id)
    .bind(&conn.provider)
    .bind(&conn.provider_user_id)
    .bind(&conn.provider_email)
    .bind(&conn.access_token)
    .bind(&conn.refresh_token)
    .fetch_one(pool)
    .await
}

/// Get OAuth connection by provider and provider user ID
pub async fn get_oauth_connection(
    pool: &DbPool,
    provider: &str,
    provider_user_id: &str,
) -> Result<Option<OAuthConnection>, sqlx::Error> {
    sqlx::query_as::<_, OAuthConnection>(
        r#"
        SELECT id, user_id, provider, provider_user_id, provider_email, access_token, refresh_token, created_at, updated_at
        FROM oauth_connections
        WHERE provider = $1 AND provider_user_id = $2
        "#,
    )
    .bind(provider)
    .bind(provider_user_id)
    .fetch_optional(pool)
    .await
}

/// Get all OAuth connections for a user
pub async fn get_user_oauth_connections(pool: &DbPool, user_id: Uuid) -> Result<Vec<OAuthConnection>, sqlx::Error> {
    sqlx::query_as::<_, OAuthConnection>(
        r#"
        SELECT id, user_id, provider, provider_user_id, provider_email, access_token, refresh_token, created_at, updated_at
        FROM oauth_connections
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Update OAuth tokens
pub async fn update_oauth_tokens(
    pool: &DbPool,
    connection_id: Uuid,
    access_token: Option<&str>,
    refresh_token: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE oauth_connections
        SET access_token = COALESCE($2, access_token),
            refresh_token = COALESCE($3, refresh_token),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(connection_id)
    .bind(access_token)
    .bind(refresh_token)
    .execute(pool)
    .await?;
    Ok(())
}

/// Create a new email token (verification or reset)
pub async fn create_email_token(pool: &DbPool, token: &NewEmailToken) -> Result<EmailToken, sqlx::Error> {
    // First, invalidate any existing tokens of the same type for this user
    sqlx::query(
        r#"
        DELETE FROM email_tokens
        WHERE user_id = $1 AND token_type = $2
        "#,
    )
    .bind(token.user_id)
    .bind(&token.token_type)
    .execute(pool)
    .await?;

    sqlx::query_as::<_, EmailToken>(
        r#"
        INSERT INTO email_tokens (user_id, token_type, token_hash, expires_at)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, token_type, token_hash, expires_at, used_at, created_at
        "#,
    )
    .bind(token.user_id)
    .bind(&token.token_type)
    .bind(&token.token_hash)
    .bind(token.expires_at)
    .fetch_one(pool)
    .await
}

/// Get email token by hash (for verification)
pub async fn get_email_token_by_hash(
    pool: &DbPool,
    token_hash: &str,
    token_type: &str,
) -> Result<Option<EmailToken>, sqlx::Error> {
    sqlx::query_as::<_, EmailToken>(
        r#"
        SELECT id, user_id, token_type, token_hash, expires_at, used_at, created_at
        FROM email_tokens
        WHERE token_hash = $1 AND token_type = $2 AND used_at IS NULL AND expires_at > NOW()
        "#,
    )
    .bind(token_hash)
    .bind(token_type)
    .fetch_optional(pool)
    .await
}

/// Mark email token as used
pub async fn mark_email_token_used(pool: &DbPool, token_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE email_tokens
        SET used_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(token_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Clean up expired email tokens
pub async fn cleanup_expired_tokens(pool: &DbPool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM email_tokens
        WHERE expires_at < NOW()
        "#,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

/// Get session by user ID
pub async fn get_session_by_user_id(pool: &DbPool, user_id: Uuid) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        r#"
        SELECT id, device_id, game_state, created_at, updated_at
        FROM sessions
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

/// Create or update session for a user
pub async fn save_user_session(
    pool: &DbPool,
    user_id: Uuid,
    device_id: &str,
    game_state: &Value,
) -> Result<Session, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        r#"
        INSERT INTO sessions (user_id, device_id, game_state)
        VALUES ($1, $2, $3)
        ON CONFLICT (device_id)
        DO UPDATE SET user_id = $1, game_state = $3, updated_at = NOW()
        RETURNING id, device_id, game_state, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(device_id)
    .bind(game_state)
    .fetch_one(pool)
    .await
}

/// Update session state for a user
pub async fn update_user_session_state(
    pool: &DbPool,
    user_id: Uuid,
    state: &Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET game_state = $2, updated_at = NOW()
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .bind(state)
    .execute(pool)
    .await?;
    Ok(())
}

