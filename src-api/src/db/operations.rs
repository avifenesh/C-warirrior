//! Database CRUD operations for sessions and player progress

use super::models::{NewProgress, NewSession, PlayerProgress, SaveSlot, Session};
use super::DbPool;

use rand::Rng;
use serde_json::Value;
use std::time::Duration;

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
