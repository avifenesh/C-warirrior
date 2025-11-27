//! Database CRUD operations for sessions and player progress

 //! Database CRUD operations for sessions and player progress

use super::models::{NewSession, NewProgress, Session};
use super::DbPool;
use rand::Rng;
use std::time::Duration;
use serde_json::Value;

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

/// Create or update a session (upsert) (with retry logic)
pub async fn save_session(pool: &DbPool, session: &NewSession) -> Result<Option<Session>, sqlx::Error> {
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
pub async fn save_progress(pool: &DbPool, progress: &NewProgress) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO player_progress (device_id, level_id, completed, xp_earned, completed_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (device_id, level_id)
        DO UPDATE SET completed = $3, xp_earned = $4, completed_at = NOW()
        "#,
    )
    .bind(&progress.device_id)
    .bind(&progress.level_id)
    .bind(progress.completed)
    .bind(progress.xp_earned)
    .execute(pool)
    .await?;
    Ok(())
}
