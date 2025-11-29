# Cleanup Plan – Backend Agent (A)

Scope: Rust core logic (`src/`), HTTP API (`src-api/`), Tauri wrapper (`src-tauri/`), DB schema/operations. Do **not** edit TypeScript files; match TS contracts defined by the frontend agent.

High-level goals:
- Keep all game rules and state in Rust.
- Make HTTP API and Tauri behave the same from the UI’s perspective.
- Implement Save/Load for HTTP in a way that mirrors Tauri’s semantics.
- Reduce noise from debug logging.

See `docs/ai/cleanup-entry.md` for shared goals and coordination rules.

---

## Phase 0 – Baseline & Mapping

- [ ] Run `cargo test` in `src/` and `src-api/`; note existing failures if any (no unrelated fixes).
- [ ] Manually hit key HTTP endpoints with a dev DB:
  - `/health`
  - `/api/game/init`
  - `/api/game/state`
  - `/api/game/render-state`
  - `/api/levels`
  - `/api/levels/:id/load`
  - `/api/code/submit` (valid and invalid submissions).
- [ ] Confirm Tauri command registration in `src-tauri/src/main.rs` matches functions in `src-tauri/src/commands/*.rs`.

---

## Phase 1 – Dead Code & Interface Pruning

Objective: remove unused commands and helpers to simplify later work.

- [ ] Verify that the Tauri `complete_level` command is not invoked by the frontend:
  - Command: `src-tauri/src/commands/progress.rs::complete_level`.
  - Registration: `src-tauri/src/main.rs` (generate_handler list).
- [ ] If confirmed unused:
  - [ ] Remove `complete_level` from `src-tauri/src/commands/progress.rs` (or delete the file if it holds only that command).
  - [ ] Remove it from the `invoke_handler` list in `src-tauri/src/main.rs`.
  - [ ] Remove any now-dead module wiring in `src-tauri/src/commands/mod.rs` or similar.
- [ ] Re-run `cargo test` and `cargo tauri dev` to ensure no remaining references to `complete_level`.

Note: Frontend agent will remove the corresponding TS interface and `Backend.completeLevel`; do not touch TS.

---

## Phase 2 – Type Alignment (CodeResult & Events)

Objective: ensure Rust and HTTP responses match the TS types owned by the frontend.

Frontend will define the canonical shapes in TS:
- `CodeResult` fields (minimum): success, stdout, stderr, compile_error, execution_time_ms, feedback, optional hint, optional render_state, optional xp_earned, optional doors_unlocked.
- `LevelCompleteEvent` fields: level_id, xp_earned, next_level_id (optional), newly_unlocked (optional).

Backend tasks:

- [ ] Update Tauri `CodeResult` in `src-tauri/src/commands/code.rs` to match the TS shape:
  - Keep core execution fields.
  - Include any extra fields agreed with frontend (e.g. `doors_unlocked`, optional `xp_earned`).
  - If the frontend expects `render_state`, either include it or rely on a follow-up `get_render_state` call and leave it out; align with the TS definition.
- [ ] Ensure HTTP `SubmitCodeResponse` in `src-api/src/main.rs` has fields that can be mapped one-to-one to TS `CodeResult`:
  - If you keep a separate Rust struct (`SubmitCodeResponse`), align field names and semantics.
- [ ] Update Tauri `LevelCompleteEvent` in `src-tauri/src/commands/code.rs`:
  - Add `next_level_id` if the frontend requires it, computed via `LevelRegistry::get_next_level`.
  - Retain `newly_unlocked` if useful; frontend can ignore extra fields.
- [ ] Confirm serialization derives (`Serialize`) are present so shapes are exported to the frontend correctly.

Do not change TS definitions; track against TS shape described in the entry/frontend spec.

---

## Phase 3 – Backend Parity (Core Flow & Hints)

Objective: bring HTTP API up to parity with Tauri for core gameplay flow.

- [ ] Add current-level data endpoint to HTTP:
  - [ ] In `src-api/src/main.rs`, add route `GET /api/levels/current`:
    - Get `GameState` using `get_or_create_session`.
    - Use `state.levels.get_level(current_level_id)` to fetch `LevelData`.
    - Return `LevelData` as JSON or 400/404 if no level is loaded.
- [ ] Add hint endpoint to HTTP:
  - [ ] In `src-api/src/main.rs`, add route like `GET /api/code/hint/:index`:
    - Use device’s current `GameState.current_level_id`.
    - Look up `LevelData.hints[index]`, same indexing behavior as Tauri `get_hint`.
    - Return the hint string or 404/400 when there is no current level or hint.
- [ ] Align `PlayerAction::Resume` logic with Tauri:
  - [ ] In `src-api/src/main.rs::process_action`, change `PlayerAction::Resume` to:
    - Only set `game_phase = GamePhase::Playing` when current phase is `Paused` or `Coding`, not e.g. `LevelComplete`.
- [ ] Confirm HTTP `process_action` delegates only to `GameState` methods (`move_player`, `interact_with_nearest`, etc.) and does not embed extra game rules.

After this phase, HTTP and Tauri should behave the same for movement, pause/resume, and hints.

---

## Phase 4 – Save/Load for HTTP Backend

Objective: add Save/Load support to HTTP backend similar to Tauri’s `SaveManager`.

Data model (you can adjust details, but keep it compatible with frontend’s `SaveSlot` type: id/name/progress/timestamp):

- [ ] In `src-api/src/db/schema.rs`, add a `save_slots` table with fields:
  - `id` (UUID primary key)
  - `device_id` (text, indexed)
  - `slot_name` (text)
  - `save_data` (jsonb) – serialized snapshot (e.g. `GameState` + key progression fields).
  - `created_at` / `updated_at` timestamps.
- [ ] In `src-api/src/db/models.rs`, add corresponding structs:
  - `SaveSlot` (for rows).
  - `NewSaveSlot` (for inserts).
- [ ] In `src-api/src/db/operations.rs`, add helper functions:
  - `upsert_save_slot(pool, device_id, slot_name, save_data)`.
  - `get_save_slot(pool, device_id, slot_name)`.
  - `delete_save_slot(pool, device_id, slot_name)`.
  - `list_save_slots(pool, device_id)`.

Routes in `src-api/src/main.rs`:

- [ ] Add `GET /api/saves`:
  - Uses `DeviceId` extension.
  - Returns an array of save-slot metadata compatible with frontend `SaveSlot` (name, timestamp, progress, etc.).
- [ ] Add `POST /api/saves/:slot`:
  - Serializes current `GameState` (and derived progression info) into `save_data`.
  - Calls `upsert_save_slot`.
- [ ] Add `GET /api/saves/:slot`:
  - Fetches `save_data` for the device/slot.
  - Restores into a `GameState` and replaces the in-memory + DB session.
- [ ] Add `DELETE /api/saves/:slot`:
  - Deletes the slot for that device.
- [ ] Optional: `POST /api/saves/autosave` that writes to a reserved `slot_name`.

Tauri path:

- [ ] Keep using `code_warrior::persistence::SaveManager` in `src-tauri/src/commands/save.rs`.
- [ ] Ensure its `SaveSlotInfo` fields are easily mapped to the TS `SaveSlot` type (slot name, total XP, levels completed, current level, timestamp).

Frontend will wire the TS `Backend` methods and `SaveLoad.svelte` to these routes.

---

## Phase 5 – Progress Exposure

Objective: expose progression data so the frontend can show real progress instead of placeholders.

Choose an approach and keep it consistent across HTTP and Tauri:

Option 1 – Dedicated progress endpoint/command (recommended):

- [ ] Tauri:
  - [ ] Add `get_progress` command that returns a struct with at least:
    - `total_xp: u32`
    - `completed_levels: Vec<String>`
    - `current_level: Option<String>`
  - [ ] Compute from `GameState.progression`.
- [ ] HTTP:
  - [ ] Add `GET /api/player/progress` endpoint that:
    - Reads `player_progress` row for the device via `db::get_*` (or equivalent).
    - Returns the same shape as Tauri `get_progress`.

Option 2 – Embed minimal progress into `RenderState`:

- [ ] If you choose this path, extend `RenderState` in `src/game/state.rs` with minimal progress fields and ensure both HTTP and Tauri serialize them. Coordinate with frontend before changing the shape.

Stick to one option and document which is used so the frontend agent can consume it.

---

## Phase 6 – Events & Game Tick (Optional)

Objective: clarify or implement real-time update behavior for Tauri; HTTP remains polling-based.

- [ ] Confirm where Tauri emits events:
  - `game_tick`
  - `code_output`
  - `level_complete` (already emitted in `commands/code.rs`)
  - `game_error`
- [ ] If `game_tick` is not emitted yet and you want real-time Tauri updates:
  - [ ] Consider adding a simple timer-driven tick that:
    - Calls `GameState.update(delta)` periodically.
    - Emits a `game_tick` event with `RenderState`.
- [ ] Otherwise, document that Tauri currently relies on explicit commands (like HTTP) and that events are limited to level completion and errors.

Frontend will keep HTTP on polling (via `/api/game/render-state`) and use events only on Tauri where available.

---

## Phase 7 – Logging Cleanup

Objective: remove noisy debug logs from Tauri/backend code.

- [ ] Replace `println!` debug statements in `src-tauri/src/commands/code.rs` with `tracing::debug!` or remove them:
  - Entry logs like `"[submit_code] Command received"`.
  - Per-call logs that are only useful during local debugging.
- [ ] Consider replacing `eprintln!` in `src-tauri/src/commands/levels.rs` with `tracing::warn!` (for map load failures).
- [ ] Ensure HTTP API uses `tracing` exclusively and contains no stray `println!` or `dbg!` calls.

After each phase, run appropriate tests and basic manual flows (see AGENTS.md) before moving on.

