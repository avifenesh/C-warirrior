# Code Warrior Cleanup & Feature Parity Plan (Two Agents)

This document coordinates two AI agents working in parallel to clean up the codebase, align types, and add full web Save/Load support.

Terminology:
- **Agent A (Backend)** – Rust core (`src/`), HTTP API (`src-api/`), Tauri commands (`src-tauri/`), DB schema/ops.
- **Agent B (Frontend)** – Svelte/TS (`src-ui/`), TS backend abstraction, UI wiring, small doc tweaks.

## Coordination Rules

- **Phase order**: Work roughly in phase order; do not start a later phase that depends on earlier types/routes before those are merged.
- **File ownership per phase**: Only the agent listed for a file edits it in that phase. If the other agent needs a change, they add a TODO entry under that agent’s tasks here instead of editing directly.
- **Shared files**: `src-ui/src/lib/types.ts`, `src-ui/src/lib/backend/types.ts`, and `src-api/src/main.rs` are shared. When a task touches these, only one agent edits them in that phase; the counterpart adapts their code to the new shape afterward.
- **Status tracking**: Each agent may add `- [x]` or `- [ ]` markers next to tasks they complete. Do not delete tasks; mark them done.
- **Testing**: After each phase that changes behavior, Agent A runs `cargo test` and HTTP API smoke tests; Agent B runs `npm test` (if applicable) and manual web+Tauri flows per AGENTS.md.

Phases below are meant to be bite-sized; agents should complete one sub-batch at a time and validate before moving on.

---

## Current Status Snapshot

This document is now primarily for **context** and **remaining work**, not a history log.

At a high level, desktop and web are already aligned for core gameplay, types, Save/Load, and progress. The remaining joint cleanup items are:

- **Baseline validation**
  - Backend agent runs `cargo test` + HTTP smoke tests and surfaces any failing cases.
  - Frontend agent runs full web + Tauri flows and notes any regressions separately.
- **Events & tick behavior (Tauri)**
  - Decide whether to implement a `game_tick` loop on desktop or to explicitly document that desktop currently uses request/response only (with level-completion/error events).
  - Update docs so frontend agents know what events to expect.
- **Feature placeholders (future feature work)**
  - Inventory actions and achievements should be treated as backlog features and addressed with fresh, focused specs when they become priority.

Use the per-agent files for concrete task lists:
- Backend: `docs/ai/cleanup-agent-backend.md`
- Frontend: `docs/ai/cleanup-agent-frontend.md`

## Phase 0 – Baseline & Mapping

Goal: Confirm current behavior and ensure both agents understand the existing wiring before edits.

**Agent A (Backend)**
- [ ] Run `cargo test` in `src/` and `src-api/` and note any existing failures (do not fix unrelated ones).
- [ ] Hit key HTTP endpoints against a dev DB: `/health`, `/api/game/init`, `/api/game/state`, `/api/game/render-state`, `/api/levels`, `/api/levels/:id/load`, `/api/code/submit` (happy path).
- [ ] Confirm Tauri commands registered in `src-tauri/src/main.rs` match function definitions in `src-tauri/src/commands/*.rs`.

**Agent B (Frontend)**
- [ ] Verify the primary flow on web: load app → main menu → start first level → move with WASD → open terminal → submit working C code → see level complete modal → go to next level.
- [ ] Note any runtime errors/warnings in browser console that are clearly from placeholders or missing wiring.
- [ ] Confirm `getBackend()` correctly selects Tauri vs HTTP in `src-ui/src/lib/backend/index.ts` (no runtime errors on web).

---

## Phase 1 – Dead Code & Interface Pruning

Goal: Remove unused APIs and commands so later work is simpler and less ambiguous.

**Agent B (Frontend)**
- [ ] Delete `src-ui/src/lib/api.ts` (old direct Tauri API) and ensure no imports exist (confirmed via `rg "from '\\$lib/api'"` and TS build).
- [ ] In `src-ui/src/lib/backend/types.ts`, remove `completeLevel(): Promise<LevelCompleteResult>` from the `Backend` interface.
- [ ] In `src-ui/src/lib/backend/tauri.ts`, remove the `completeLevel` method from the returned object.
- [ ] In `src-ui/src/lib/backend/http.ts`, remove the `completeLevel` stub implementation.
- [ ] In `src-ui/src/lib/types.ts`, remove the `LevelCompleteResult` interface if it is indeed unused outside the deprecated paths. If documentation relies on it, replace references with in-line types in docs instead of code.

**Agent A (Backend)**
- [ ] Remove the Tauri `complete_level` command if it is no longer needed:
  - [ ] Delete or decommission `src-tauri/src/commands/progress.rs` (or at least the `complete_level` function) if nothing calls it.
  - [ ] Remove `complete_level` from the `invoke_handler` list in `src-tauri/src/main.rs`.
  - [ ] If `src/commands/progress.rs` exists only for the Tauri command wiring, either delete it or clearly mark it as an internal helper not exposed as a command.
- [ ] Re-run tests and a simple Tauri dev run to confirm nothing is still trying to invoke `complete_level`.

---

## Phase 2 – Type Alignment (CodeResult, Events)

Goal: Make Rust and TS agree on response/event shapes so both backends behave identically from the UI’s perspective.

**Decision (both agents, spec-level)**
- [ ] Finalize `CodeResult` shape used by frontend (and both backends). Proposed fields:
  - `success: bool`
  - `stdout: string`
  - `stderr: string`
  - `compile_error: string | null`
  - `execution_time_ms: u64`
  - `feedback: string`
  - `hint: string | null` (optional)
  - `render_state: RenderState` (optional, may be `null` on Tauri if not needed)
  - `xp_earned: number` (optional, 0 if already completed)
  - `doors_unlocked: bool` (optional, UI can ignore if not needed)
- [ ] Finalize `LevelCompleteEvent` shape. Proposed fields:
  - `level_id: string`
  - `xp_earned: number`
  - `next_level_id: string | null`
  - `newly_unlocked: string[]`

**Agent B (Frontend)**
- [ ] Update `src-ui/src/lib/types.ts` to match the agreed `CodeResult` and `LevelCompleteEvent` shapes.
- [ ] Search all uses of `CodeResult` and adapt UI expectations (e.g., optional vs required fields). Key file: `src-ui/src/routes/+page.svelte` (uses `render_state`, `xp_earned`).
- [ ] Update any docs in `docs/interfaces/tauri-commands.md` that reference old shapes.

**Agent A (Backend)**
- [ ] Update Tauri `CodeResult` in `src-tauri/src/commands/code.rs` to match the agreed shape, adding/removing fields as needed.
- [ ] Ensure HTTP `SubmitCodeResponse` in `src-api/src/main.rs` can be trivially mapped to TS `CodeResult` (either rename it to `CodeResult` or keep as is but with matching fields).
- [ ] Update Tauri `LevelCompleteEvent` struct in `src-tauri/src/commands/code.rs` to include `next_level_id` and use the unified shape.
- [ ] If events are unused on web, keep HTTP-side event stubs in `http.ts` as no-ops, but ensure Tauri emits events consistent with the new shape.

---

## Phase 3 – Backend Parity (Game Flow & Hints)

Goal: Ensure HTTP and Tauri backends expose equivalent behavior for core game flow and hints.

**Agent A (Backend)**
- [ ] In `src-api/src/main.rs`, add a level data endpoint:
  - [ ] Route `GET /api/levels/current` that:
    - Resolves the current `GameState` for the device via `get_or_create_session`.
    - Uses `state.levels.get_level(current_level_id)` to fetch `LevelData` and returns it as JSON.
- [ ] In `src-api/src/main.rs`, add a hint endpoint:
  - [ ] Route `GET /api/code/hint/:index` that uses current level ID + `LevelData.hints[index]`, mirroring `get_hint` in Tauri.
- [ ] In `src-api/src/main.rs::process_action`, change `PlayerAction::Resume` handling to match Tauri:
  - Only set `game_phase = GamePhase::Playing` when the current phase is `Paused` or `Coding`, not when `LevelComplete`.
- [ ] Ensure HTTP `process_action` delegates purely to `GameState` methods (`move_player`, `interact_with_nearest`, etc.) and does not introduce extra game rules.

**Agent B (Frontend)**
- [ ] In `src-ui/src/lib/backend/http.ts`, update `getLevelData()` to call `/api/levels/current` instead of relying solely on `currentLevelData` cache (but keep cache as a fast-path if desired).
- [ ] In `http.ts`, update `getHint(hintIndex)` to call the new `/api/code/hint/:index` endpoint instead of directly reading `currentLevelData.hints`.
- [ ] Verify `+page.svelte` hint behavior still works for both Tauri and HTTP backends.

---

## Phase 4 – Save/Load Parity for Web & Desktop

Goal: Implement Save/Load on HTTP backend and wire the existing Save/Load UI for both web and Tauri.

**Agent A (Backend – HTTP & core)**
- [ ] Design and implement a `save_slots` table in `src-api/src/db/schema.rs` (or reuse existing structures). Suggested fields:
  - `id: uuid`
  - `device_id: text`
  - `slot_name: text`
  - `save_data: jsonb` (serialized `SaveData`-like structure or `GameState` + `ProgressionState`)
  - `created_at`, `updated_at` timestamps
- [ ] In `src-api/src/db/models.rs`, add `SaveSlot` and `NewSaveSlot` structs.
- [ ] In `src-api/src/db/operations.rs`, add operations:
  - `upsert_save_slot(pool, device_id, slot_name, save_data)`
  - `get_save_slot(pool, device_id, slot_name)`
  - `delete_save_slot(pool, device_id, slot_name)`
  - `list_save_slots(pool, device_id)`
- [ ] In `src-api/src/main.rs`, add routes:
  - `GET /api/saves` → list save slots for the current device.
  - `GET /api/saves/:slot` → load save slot and return serialized save data.
  - `POST /api/saves/:slot` → create/update save slot using current `GameState` + derived progression.
  - `DELETE /api/saves/:slot` → delete save slot.
  - (Optional) `POST /api/saves/autosave` → write to a reserved slot name.
- [ ] Define a JSON shape for HTTP save slots compatible with the frontend’s `SaveSlot` interface (id/name/progress/timestamp).

**Agent A (Backend – Tauri)**
- [ ] Keep using `code_warrior::persistence::SaveManager` for desktop, but ensure its `SaveSlotInfo` can be mapped into the same TS `SaveSlot` structure used by HTTP (fields: id/slot_name, timestamp, progress string, etc.).

**Agent B (Frontend)**
- [ ] Extend `Backend` interface in `src-ui/src/lib/backend/types.ts` with save-related methods:
  - `listSaves(): Promise<SaveSlot[]>`
  - `saveGame(slotId: string): Promise<void | SaveResult>`
- [ ] Implement these in `src-ui/src/lib/backend/tauri.ts` by calling existing Tauri commands:
  - `list_saves`, `save_game`, `load_game`, `delete_save`, `autosave`.
- [ ] Implement these in `src-ui/src/lib/backend/http.ts` using the new HTTP routes.
- [ ] Wire `src-ui/src/lib/components/SaveLoad.svelte` into the main UI (e.g., via a menu or HUD):
  - Maintain a `slots: SaveSlot[]` state populated from `backend.listSaves()`.
  - Handle `on:save`, `on:load`, and `on:delete` by calling the backend methods.
  - Remove or update the “Local only (UI placeholder)” text once real persistence is working on both platforms.
- [ ] Ensure behavior is consistent between Tauri and web (slots list, timestamps, error handling).

---

## Phase 5 – Progress & ProgressTracker Integration

Goal: Replace placeholder progress values in the UI with real data from backend progression.

**Agent A (Backend)**
- [ ] Decide how to expose progress:
  - Option 1: Explicit progress endpoint/command (recommended):
    - Tauri: `get_progress` command returning `total_xp`, `completed_levels`, `current_level` (derived from `ProgressionState`).
    - HTTP: `GET /api/player/progress` returning the same, derived from `player_progress` table.
  - Option 2: Embed minimal progress stats into `RenderState` (requires making sure they serialize cleanly).
- [ ] Implement chosen approach in both Tauri and HTTP so the data shape is identical.

**Agent B (Frontend)**
- [ ] Update `src-ui/src/routes/+page.svelte` to fetch real progress data and pass it into `ProgressTracker`:
  - Replace `completedLevels={[]}` with actual completed level IDs.
  - Replace `totalXP={1000}` with real total XP.
  - Use `getNextLevelId()` to derive the “next” label in `ProgressTracker` details instead of the hardcoded `MEMORY_ALLOC` placeholder.
- [ ] Guard against division by zero in `ProgressTracker.svelte` when `totalLevels === 0` or `totalXP === 0`.

---

## Phase 6 – Events & Game Tick (Optional Enhancement)

Goal: Clarify how real-time updates are delivered; optional but good to document.

**Agent A (Backend)**
- [ ] Confirm whether Tauri emits `game_tick`, `code_output`, `game_error` events anywhere other than `level_complete` (code completion). If not, either:
  - Add a simple tick emission (e.g., timer-based) that calls `GameState.update`, or
  - Document that desktop currently relies on request/response, similar to HTTP.

**Agent B (Frontend)**
- [ ] Ensure `onGameTick`, `onCodeOutput`, `onLevelComplete`, and `onGameError` are used consistently:
  - Tauri: events hook into `GameWorld` and UI where available.
  - HTTP: keep stubs but make sure UI doesn’t rely on them for correctness (all needed state must be available from responses & polling).

---

## Phase 7 – Logging & Debug Output Cleanup

Goal: Remove noisy debug output from production builds while keeping useful error logging.

**Agent B (Frontend)**
- [ ] Remove or downgrade `console.log` / `console.debug` statements that are only useful during development:
  - `src-ui/src/lib/backend/index.ts` – backend detection logs.
  - `src-ui/src/lib/components/GameWorld.svelte` – interact + asset load logs (keep `console.error` for failures).
  - `src-ui/src/lib/components/CodeTerminal.svelte` – focus/blur logs.
  - `src-ui/src/lib/engine/assets.ts` – `console.debug` for preload completion (optional: keep behind a feature flag).
- [ ] Ensure that remaining logs are either `console.error` for genuine errors or clearly guarded by a debug flag.

**Agent A (Backend)**
- [ ] Replace `println!` debug lines in `src-tauri/src/commands/code.rs` with `tracing::debug!` or remove them entirely, keeping only meaningful logs (e.g., warnings on failures).
- [ ] Review `eprintln!` in `src-tauri/src/commands/levels.rs` and consider replacing with `tracing::warn!`.
- [ ] Ensure HTTP API uses structured logging via `tracing` only (no stray `println!`).

---

This plan should give two agents clear, non-overlapping responsibilities while covering:
- Dead code removal
- Type alignment
- Backend parity (including hints and Resume logic)
- Full Save/Load parity on web and desktop
- Progress tracking integration
- Optional event/tick improvements
- Logging cleanup

Agents should update this file as they complete tasks or need to adjust ownership for any specific file in a phase.
