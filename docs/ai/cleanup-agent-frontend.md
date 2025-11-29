# Cleanup Plan – Frontend Agent (B)

Scope: Svelte/TypeScript frontend (`src-ui/**`), including backend abstraction in TS and UI components. Do **not** edit Rust code; rely on the backend agent for HTTP/Tauri changes.

High-level goals:
- Use the `Backend` abstraction cleanly (no direct HTTP or Tauri calls outside it).
- Keep TS types in sync with backend behavior described in the entry spec.
- Wire Save/Load and progress UI to real backend data.
- Remove obsolete APIs and noisy logging.

See `docs/ai/cleanup-entry.md` for shared goals and coordination rules.

---

## Phase 0 – Baseline & Mapping

- [ ] Verify main web flow:
  - Load app → main menu → start first level → move with WASD → open terminal → submit working C → see level complete → go to next level.
- [ ] Note any console errors/warnings that clearly come from placeholders or missing wiring.
- [ ] Confirm `getBackend()` in `src-ui/src/lib/backend/index.ts` selects HTTP on web (no Tauri globals) and Tauri on desktop.

---

## Phase 1 – Dead Code & Interface Pruning

Objective: remove obsolete frontend APIs and unused interfaces.

- [ ] Delete `src-ui/src/lib/api.ts` (old direct Tauri API) and confirm:
  - `rg "from '\\$lib/api'" src-ui` returns no matches.
  - TypeScript build still passes.
- [ ] In `src-ui/src/lib/backend/types.ts`:
  - [ ] Remove `completeLevel(): Promise<LevelCompleteResult>` from the `Backend` interface.
- [ ] In `src-ui/src/lib/backend/tauri.ts`:
  - [ ] Remove the `completeLevel` method from the returned object.
- [ ] In `src-ui/src/lib/backend/http.ts`:
  - [ ] Remove the `completeLevel` stub that throws an error.
- [ ] In `src-ui/src/lib/types.ts`:
  - [ ] Remove the `LevelCompleteResult` interface if it is only used by the deprecated `completeLevel` path.

If you find any live use of these types or methods, stop and add a TODO note for the backend agent to revisit; do not partially wire them.

---

## Phase 2 – Type Alignment (CodeResult & Events)

Objective: define canonical TS shapes for results/events and keep them stable for backend to match.

In `src-ui/src/lib/types.ts`:

- [ ] Update `CodeResult` to the agreed shape (minimum fields):
  - `success: boolean`
  - `stdout: string`
  - `stderr: string`
  - `compile_error: string | null`
  - `execution_time_ms: number`
  - Optional fields:
    - `feedback?: string`
    - `hint?: string | null`
    - `render_state?: RenderState`
    - `xp_earned?: number`
    - `doors_unlocked?: boolean`
- [ ] Update `LevelCompleteEvent`:
  - `level_id: string`
  - `xp_earned: number`
  - `next_level_id: string | null`
  - `newly_unlocked?: string[]`

Usage updates:

- [ ] In `src-ui/src/routes/+page.svelte`, adapt logic consuming `CodeResult`:
  - Only rely on fields that are documented (e.g., use `render_state` if present; otherwise call `getRenderState`).
  - Use `xp_earned` when available for UX (toasts, progress), but handle `undefined` gracefully.
- [ ] If any component uses `LevelCompleteEvent`, update it to work with the new fields; otherwise keep the type aligned for future event use.

Do not change Rust; if you need additional fields from backend, document them and coordinate via backend spec.

---

## Phase 3 – Backend Parity Wiring (Core Flow & Hints)

Objective: consume new HTTP routes and keep behavior consistent across backends.

In `src-ui/src/lib/backend/http.ts`:

- [ ] `getLevelData()`:
  - [ ] Change to call `GET /api/levels/current` (backend will implement this).
  - [ ] Keep `currentLevelData` cache as a fast path if helpful, but treat backend response as the source of truth.
- [ ] `getHint(hintIndex: number)`:
  - [ ] Change to call `GET /api/code/hint/:index` instead of reading `this.currentLevelData.hints[index]` directly.
  - [ ] Preserve existing error behavior (throw when no more hints).

In `src-ui/src/routes/+page.svelte`:

- [ ] Verify `getNextHint()` uses `backend.getHint` and handles “no more hints” by showing a toast or message.
- [ ] Confirm hint flow works on both Tauri and HTTP paths.

If HTTP routes are not yet available and cause errors, add a TODO for backend agent and temporarily guard calls with clear error messaging, not silent failures.

---

## Phase 4 – Save/Load UI Wiring (Both Platforms)

Objective: wire the existing Save/Load UI to real backend Save/Load operations for web and desktop.

Types & backend abstraction:

- [ ] Define or confirm `SaveSlot` TS interface (if not already present) in a suitable place (e.g. `src-ui/src/lib/types.ts` or next to `SaveLoad.svelte`), with fields like:
  - `id: string` (or `slot_name`)
  - `name: string`
  - `timestamp: string`
  - `progress: string`
  - `empty?: boolean`
- [ ] Extend `Backend` interface in `src-ui/src/lib/backend/types.ts` to include:
  - `listSaves(): Promise<SaveSlot[]>`
  - `saveGame(slotId: string): Promise<void>` (and optionally `deleteSave`, `loadGame` if you expose them directly).

Implementations:

- [ ] In `src-ui/src/lib/backend/tauri.ts`:
  - [ ] Implement `listSaves`, `saveGame`, `loadGame`, `deleteSave` by calling existing Tauri commands (`list_saves`, `save_game`, `load_game`, `delete_save`, `autosave`).
  - [ ] Map `SaveSlotInfo` from Rust to the TS `SaveSlot` shape.
- [ ] In `src-ui/src/lib/backend/http.ts`:
  - [ ] Implement `listSaves`, `saveGame`, `loadGame`, `deleteSave` using the HTTP routes that backend agent will add (`/api/saves`, `/api/saves/:slot`, etc.).

UI wiring:

- [ ] Integrate `src-ui/src/lib/components/SaveLoad.svelte` into the main UI (for example, via a menu or HUD panel).
  - Maintain a `slots: SaveSlot[]` state populated at boot or on user request using `backend.listSaves()`.
  - Handle component events:
    - `on:save` → call `backend.saveGame(id)` then refresh `slots`.
    - `on:load` → call backend load method then refresh render state or reload level.
    - `on:delete` → call backend delete method then refresh `slots`.
- [ ] Remove or update the “Local only (UI placeholder)” text once Save/Load works on both platforms.

---

## Phase 5 – Progress & ProgressTracker

Objective: feed real progress info into the `ProgressTracker` instead of placeholders.

Assuming backend exposes either:
- A dedicated progress endpoint/command, or
- Extra progress fields in `RenderState`.

Tasks:

- [ ] Add a small frontend helper to fetch progress:
  - For example, `backend.getProgress()` if the backend exposes a dedicated method.
  - Or derive from `renderState` if progress is embedded there.
- [ ] In `src-ui/src/routes/+page.svelte`:
  - [ ] Replace `completedLevels={[]}` with the actual list of completed level IDs.
  - [ ] Replace `totalXP={1000}` with real total XP.
  - [ ] Use `getNextLevelId()` (already present) to compute the “next” label instead of the hardcoded `MEMORY_ALLOC` placeholder in `ProgressTracker.svelte`.
- [ ] Update `src-ui/src/lib/components/ProgressTracker.svelte`:
  - [ ] Guard against division by zero when `totalLevels === 0` or `totalXP === 0` (clamp percent to 0).

If backend shape changes are needed, coordinate by updating this spec and the backend spec together.

---

## Phase 6 – Events & Game Tick (Optional)

Objective: keep event usage clear and safe.

- [ ] In `src-ui/src/lib/backend/tauri.ts`:
  - [ ] Ensure `onGameTick`, `onCodeOutput`, `onLevelComplete`, `onGameError` all return proper unsubscribe functions and handle missing events gracefully.
- [ ] In `src-ui/src/lib/backend/http.ts`:
  - [ ] Keep stubs for `onCodeOutput`, `onLevelComplete`, `onGameError` as no-ops; ensure UI does not rely on them.
- [ ] In `src-ui/src/routes/+page.svelte`:
  - [ ] Confirm `bindEvents()` works for both backends and does not throw when handlers are stubs.

Do not add new event types without documenting them and coordinating with backend.

---

## Phase 7 – Logging Cleanup

Objective: remove noisy console logging while keeping meaningful error logs.

- [ ] In `src-ui/src/lib/backend/index.ts`:
  - [ ] Remove backend-detection `console.log` calls (Tauri detection and backend choice). If needed, keep a single debug log behind a flag.
- [ ] In `src-ui/src/lib/components/GameWorld.svelte`:
  - [ ] Remove `console.log` calls for interact events and asset load success.
  - [ ] Keep `console.error` logs for asset load failures and render errors.
- [ ] In `src-ui/src/lib/components/CodeTerminal.svelte`:
  - [ ] Remove focus/blur `console.log` calls.
- [ ] In `src-ui/src/lib/engine/assets.ts`:
  - [ ] Remove `console.debug` for preload completion or guard it behind a debug flag.

After each phase, re-run the main web and Tauri flows to ensure the UI still behaves correctly. Do not add new logging unless it directly supports error diagnosis. 

