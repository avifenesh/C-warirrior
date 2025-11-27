# Agent A: Database Persistence

## Role
Wire the existing `db` module to actively persist game sessions and player progress.

## Context
Round 1 created the `db` module with full CRUD operations, but the API still uses in-memory `SessionStore` (HashMap). Your job is to connect these.

## Files You Own (LOCK these)
- `src-api/src/main.rs` (modify handlers to use db operations)
- `src-api/src/db/operations.rs` (may need adjustments)

## Files to READ ONLY
- `src-api/src/db/mod.rs`
- `src-api/src/db/schema.rs`
- `src-api/src/db/models.rs`

## Tasks
1. **Read `.agents/COORDINATION.md`** - Check lock table before starting
2. **Lock your files** - Update COORDINATION.md
3. **Modify `init_game` handler**:
   - Call `db::operations::create_session()` to persist new sessions
   - Store the session UUID, return it as device_id
4. **Modify `get_game_state` handler**:
   - Look up session from database first
   - Fall back to in-memory if not found (for backward compat)
5. **Modify `submit_code` handler**:
   - On success, call `db::operations::complete_level()`
   - Call `db::operations::add_xp()` to persist XP gains
6. **Add session recovery endpoint** (optional):
   - `GET /api/game/session/:device_id` - Resume from DB
7. **Test**: `cargo check` and `cargo build`
8. **Update lock table** - Mark files as DONE

## DO NOT
- Modify database schema
- Touch frontend files
- Change API response shapes (keep backward compatible)

## Completion Criteria
- [ ] Sessions persist to PostgreSQL
- [ ] Level completions persist to player_progress
- [ ] XP gains persist
- [ ] `cargo build` passes
- [ ] Lock table updated
