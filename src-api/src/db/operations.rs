//! Database CRUD operations for sessions and player progress

use super::models::{NewProgress, NewSession, PlayerProgress, Session};
use super::DbPool;
use serde_json::Value;

/// Get a session by device ID
pub async fn get_session(pool: &DbPool, device_id: &str) -> Result<Option<Session>, sqlx::Error> {
    sqlx::query_as::<_, Session>(
        r#"
        SELECT id, device_id, game_state, created_at, updated_at
        FROM sessions
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .fetch_optional(pool)
    .await
}

/// Create or update a session (upsert)
pub async fn save_session(pool: &DbPool, session: &NewSession) -> Result<Session, sqlx::Error> {
    sqlx::query_as::<_, Session>(
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
}

/// Update only the game state for a session
pub async fn update_session_state(
    pool: &DbPool,
    device_id: &str,
    state: &Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE sessions
        SET game_state = $2, updated_at = NOW()
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .bind(state)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a session by device ID
pub async fn delete_session(pool: &DbPool, device_id: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        DELETE FROM sessions WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// Get player progress by device ID
pub async fn get_progress(
    pool: &DbPool,
    device_id: &str,
) -> Result<Option<PlayerProgress>, sqlx::Error> {
    sqlx::query_as::<_, PlayerProgress>(
        r#"
        SELECT id, device_id, completed_levels, total_xp, current_level, achievements, created_at, updated_at
        FROM player_progress
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .fetch_optional(pool)
    .await
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

/// Add XP to player progress
pub async fn add_xp(pool: &DbPool, device_id: &str, xp: i32) -> Result<i32, sqlx::Error> {
    let row: (i32,) = sqlx::query_as(
        r#"
        UPDATE player_progress
        SET total_xp = total_xp + $2, updated_at = NOW()
        WHERE device_id = $1
        RETURNING total_xp
        "#,
    )
    .bind(device_id)
    .bind(xp)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

/// Mark a level as completed
pub async fn complete_level(
    pool: &DbPool,
    device_id: &str,
    level_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE player_progress
        SET completed_levels = array_append(completed_levels, $2), updated_at = NOW()
        WHERE device_id = $1 AND NOT ($2 = ANY(completed_levels))
        "#,
    )
    .bind(device_id)
    .bind(level_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Grant an achievement
pub async fn grant_achievement(
    pool: &DbPool,
    device_id: &str,
    achievement: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE player_progress
        SET achievements = array_append(achievements, $2), updated_at = NOW()
        WHERE device_id = $1 AND NOT ($2 = ANY(achievements))
        "#,
    )
    .bind(device_id)
    .bind(achievement)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

/// Set current level
pub async fn set_current_level(
    pool: &DbPool,
    device_id: &str,
    level_id: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE player_progress
        SET current_level = $2, updated_at = NOW()
        WHERE device_id = $1
        "#,
    )
    .bind(device_id)
    .bind(level_id)
    .execute(pool)
    .await?;
    Ok(())
}
