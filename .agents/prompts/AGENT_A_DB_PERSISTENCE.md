# Agent A: DB Handlers

## Goal
Wire the existing db operations to API handlers so game state actually persists to PostgreSQL.

## Status: 🔄 IN PROGRESS

## Already Done
- [x] Neon-optimized pool settings (max_connections=3, timeouts)
- [x] Retry logic with exponential backoff in operations.rs
- [x] Removed redundant create_pool function

## Remaining Tasks
1. [ ] In `init_game` handler: Call `db::operations::save_session()` to persist new sessions
2. [ ] In `get_game_state` handler: Call `db::operations::get_session()` to load from DB
3. [ ] In `submit_code` handler: Call `db::operations::complete_level()` on success
4. [ ] Run `cargo build` - must pass
5. [ ] Test: Create session, verify it persists across server restart

## Files
- **Modify**: `src-api/src/main.rs` (handlers)
- **Reference**: `src-api/src/db/operations.rs` (available functions)

## Available DB Operations
```rust
// From src-api/src/db/operations.rs:
pub async fn get_session(pool: &DbPool, device_id: &str) -> Result<Option<Session>, anyhow::Error>
pub async fn save_session(pool: &DbPool, session: &NewSession) -> Result<Option<Session>, anyhow::Error>
pub async fn complete_level(pool: &DbPool, session_id: Uuid, level_id: &str, xp_earned: i32) -> Result<PlayerProgress, anyhow::Error>
pub async fn add_xp(pool: &DbPool, session_id: Uuid, xp: i32) -> Result<(), anyhow::Error>
```

## Implementation Hints

### In init_game handler:
```rust
// After creating game state, persist it
let new_session = NewSession {
    device_id: device_id.clone(),
    game_state: serde_json::to_value(&game_state)?,
};
db::operations::save_session(&pool, &new_session).await?;
```

### In get_game_state handler:
```rust
// Try to load from DB first
if let Some(session) = db::operations::get_session(&pool, &device_id).await? {
    // Deserialize and return existing state
    let game_state: GameState = serde_json::from_value(session.game_state)?;
    return Ok(game_state);
}
// Fall back to in-memory or create new
```

### In submit_code handler (on success):
```rust
// After successful code submission
db::operations::complete_level(&pool, session_id, &level_id, xp_reward).await?;
```

## Verification
```bash
cd src-api && cargo build
# Should pass with no new errors
```

## DO NOT
- Modify db/operations.rs (already done)
- Change API response formats
- Touch frontend files
