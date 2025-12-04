//! Database CRUD operations for sessions, player progress, and authentication

use super::models::{
    EmailToken, NewEmailToken, NewOAuthConnection, NewUser,
    OAuthConnection, SaveSlot, Session, User,
};
use super::DbPool;

use serde_json::Value;
use uuid::Uuid;

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
pub async fn get_user_by_username(
    pool: &DbPool,
    username: &str,
) -> Result<Option<User>, sqlx::Error> {
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
pub async fn update_user_password(
    pool: &DbPool,
    user_id: Uuid,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
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

/// Create a new OAuth connection
pub async fn create_oauth_connection(
    pool: &DbPool,
    conn: &NewOAuthConnection,
) -> Result<OAuthConnection, sqlx::Error> {
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
        SELECT id, user_id, provider, provider_user_id, provider_email, created_at, updated_at
        FROM oauth_connections
        WHERE provider = $1 AND provider_user_id = $2
        "#,
    )
    .bind(provider)
    .bind(provider_user_id)
    .fetch_optional(pool)
    .await
}

/// Create a new email token (verification or reset)
pub async fn create_email_token(
    pool: &DbPool,
    token: &NewEmailToken,
) -> Result<EmailToken, sqlx::Error> {
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
        SELECT id, user_id, token_type, expires_at, used_at, created_at
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

/// Get session by user ID
pub async fn get_session_by_user_id(
    pool: &DbPool,
    user_id: Uuid,
) -> Result<Option<Session>, sqlx::Error> {
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

// ============================================================================
// User-based operations for authenticated users (account-only progress)
// ============================================================================

/// Create or update session by user_id (upsert for authenticated users)
/// Uses a synthetic device_id derived from user_id to satisfy NOT NULL constraint
pub async fn upsert_session_by_user_id(
    pool: &DbPool,
    user_id: Uuid,
    game_state: &Value,
) -> Result<Session, sqlx::Error> {
    // Use a deterministic device_id derived from user_id for the constraint
    let synthetic_device_id = format!("user-{}", user_id);

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
    .bind(&synthetic_device_id)
    .bind(game_state)
    .fetch_one(pool)
    .await
}

/// List all save slots for a user (authenticated)
pub async fn list_save_slots_by_user_id(
    pool: &DbPool,
    user_id: Uuid,
) -> Result<Vec<SaveSlot>, sqlx::Error> {
    sqlx::query_as::<_, SaveSlot>(
        r#"
        SELECT id, device_id, slot_name, save_data, total_xp, levels_completed, current_level, created_at, updated_at
        FROM save_slots
        WHERE user_id = $1
        ORDER BY updated_at DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

/// Get a specific save slot for a user (authenticated)
pub async fn get_save_slot_by_user_id(
    pool: &DbPool,
    user_id: Uuid,
    slot_name: &str,
) -> Result<Option<SaveSlot>, sqlx::Error> {
    sqlx::query_as::<_, SaveSlot>(
        r#"
        SELECT id, device_id, slot_name, save_data, total_xp, levels_completed, current_level, created_at, updated_at
        FROM save_slots
        WHERE user_id = $1 AND slot_name = $2
        "#,
    )
    .bind(user_id)
    .bind(slot_name)
    .fetch_optional(pool)
    .await
}

/// Create or update a save slot for a user (authenticated)
pub async fn upsert_save_slot_for_user(
    pool: &DbPool,
    user_id: Uuid,
    slot_name: &str,
    save_data: &Value,
    total_xp: i32,
    levels_completed: i32,
    current_level: Option<&str>,
) -> Result<SaveSlot, sqlx::Error> {
    // Use a synthetic device_id for the constraint
    let synthetic_device_id = format!("user-{}", user_id);

    sqlx::query_as::<_, SaveSlot>(
        r#"
        INSERT INTO save_slots (user_id, device_id, slot_name, save_data, total_xp, levels_completed, current_level)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (device_id, slot_name)
        DO UPDATE SET
            user_id = $1,
            save_data = $4,
            total_xp = $5,
            levels_completed = $6,
            current_level = $7,
            updated_at = NOW()
        RETURNING id, device_id, slot_name, save_data, total_xp, levels_completed, current_level, created_at, updated_at
        "#,
    )
    .bind(user_id)
    .bind(&synthetic_device_id)
    .bind(slot_name)
    .bind(save_data)
    .bind(total_xp)
    .bind(levels_completed)
    .bind(current_level)
    .fetch_one(pool)
    .await
}

/// Delete a save slot for a user (authenticated)
pub async fn delete_save_slot_for_user(
    pool: &DbPool,
    user_id: Uuid,
    slot_name: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM save_slots
        WHERE user_id = $1 AND slot_name = $2
        "#,
    )
    .bind(user_id)
    .bind(slot_name)
    .execute(pool)
    .await?;

    Ok(())
}
