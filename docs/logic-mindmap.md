# Code Warrior Logic Mind Map (Backend + Frontend)

> Format: nested bullet hierarchy for easy parsing by AI coding agents. Each node lists the source of truth for the logic.

## Backend (Rust)
- Axum API (`src-api/src/main.rs`)
  - App state boot: connect Postgres, run migrations, load levels, create compiler, prime in-memory session cache, set CORS, start server. 【F:src-api/src/main.rs†L1-L121】
  - Session resolution: `get_or_create_session` pulls from in-memory cache, otherwise loads/creates DB record; `persist_session` writes back to DB and caches, also stores progress snapshot. 【F:src-api/src/main.rs†L240-L306】
  - Game init/render/state fetch: `/api/game/init` initializes GameState, `/api/game/state` returns raw state, `/api/game/render-state` returns render projection. 【F:src-api/src/main.rs†L308-L382】
  - Player action pipeline: `/api/game/action` deserializes `PlayerAction`, routes to movement, interaction, pause/resume, or TODO inventory/item handlers; updated state cached for subsequent calls. 【F:src-api/src/main.rs†L384-L430】
    - TODO branches: `OpenInventory` should toggle inventory UI state; `UseItem` should consume/apply inventory effects. 【F:src-api/src/main.rs†L418-L423】
  - Level listing/loading: `/api/levels` merges registry data with unlock/completion flags from `GameState`; `/api/levels/:id/load` validates unlock status, constructs `World` from level config, updates progression, persists session. 【F:src-api/src/main.rs†L432-L522】
  - Code submission (single challenge): `/api/code/submit` compiles/runs user code, validates output, completes level if successful, persists, returns feedback plus render state. 【F:src-api/src/main.rs†L522-L610】
  - Code submission (function/quest challenges): `run_function_based_challenge` executes generated harness per test case, awards XP, unlocks doors, persists state; `/api/code/submit-quest` mirrors this per quest and tracks quest completion, potentially completing the level when all quests done. 【F:src-api/src/main.rs†L612-L869】【F:src-api/src/main.rs†L896-L1016】
  - Quest/hint APIs: `/api/levels/current/quests` and `/api/levels/current/quests/:quest_id` surface quest metadata + completion flags; `/api/code/hint/:index` streams hints in order. 【F:src-api/src/main.rs†L870-L950】【F:src-api/src/main.rs†L1018-L1048】
  - Progress & saves: `/api/player/progress` derives totals from `ProgressionState`; save slots list/upsert/load using serialized `GameState`; delete endpoint removes slots. 【F:src-api/src/main.rs†L1050-L1160】
- Game core (`src/game/state.rs`)
  - GameState structure: tracks player/world/inventory/progression/active quest and phase; default spawns a 20×15 world with starter inventory. 【F:src/game/state.rs†L39-L76】
  - Level lifecycle: `start_level` sets world + player spawn and enters Playing; `complete_level` awards XP, syncs legacy totals, unlocks doors, flips to LevelComplete; `update_unlocked_levels`/`is_level_unlocked`/`is_level_completed` delegate to progression. 【F:src/game/state.rs†L78-L122】
  - Quest lifecycle: `complete_quest`, `is_quest_completed`, `get_completed_quest_count`, `is_level_fully_completed`, `maybe_complete_level` handle per-quest XP and mark level complete when all quests done. 【F:src/game/state.rs†L123-L167】
  - Mode switches: `enter_coding_mode` / `exit_coding_mode` gate movement vs terminal interaction. 【F:src/game/state.rs†L169-L175】
  - Movement & collision: `move_player` checks phase, computes displacement, rejects blocked paths via physics, updates facing. 【F:src/game/state.rs†L177-L197】
  - Interaction flow: `interact_with_nearest` searches nearby interactables; terminals set `active_quest_id` and enter coding; doors are stubbed for future logic. 【F:src/game/state.rs†L204-L238】
  - Rendering pipeline: `to_render_state` builds 20×15 viewport around player, returns tiles, offsets, phase flags, active quest for frontend. 【F:src/game/state.rs†L259-L302】
- World layout (`src/game/world.rs`)
  - Tile taxonomy & helpers for floor/wall/terminal/door/water plus quest-aware terminals. 【F:src/game/world.rs†L7-L55】
  - Default map seeds walls/borders, spawn, terminal, door, and decorative water; supports unlock_all_doors to open gates on success. 【F:src/game/world.rs†L57-L107】【F:src/game/world.rs†L128-L134】
  - Config loader `from_config` constructs worlds from level JSON including quest-linked terminals and legacy terminal positions; `get_tile_quest_id` resolves quest triggers for terminals. 【F:src/game/world.rs†L136-L189】

## Frontend (Svelte)
- App shell (`src-ui/src/routes/+page.svelte`)
  - State scaffolding: tracks backend handle, render state, level data, code draft, UI status, hints, quest metadata, and screen mode (boot/map/playing). 【F:src-ui/src/routes/+page.svelte†L16-L66】
  - Quest-reactive effects: when terminal opens and backend exposes `active_quest_id`, auto-loads quest details, resets hints/output; closes clearing quest. 【F:src-ui/src/routes/+page.svelte†L84-L115】
  - Navigation: handlers for new/continue/start first level, selecting from map, returning to map, and computing next level ID for progress tracker. 【F:src-ui/src/routes/+page.svelte†L116-L190】
  - Input dispatch: move/interact events send `PlayerAction` to backend; terminal close resumes play. 【F:src-ui/src/routes/+page.svelte†L146-L153】【F:src-ui/src/routes/+page.svelte†L186-L190】
  - Boot sequence: on mount, acquire backend (Tauri/WASM/HTTP), init game, fetch levels + progress, bind event polling, then show world map; errors surface in UI banner. 【F:src-ui/src/routes/+page.svelte†L191-L247】
  - Level loading: `startLevel` posts to backend, fetches level data + render state, resets hints/output. 【F:src-ui/src/routes/+page.svelte†L256-L277】
  - Hint pipeline: `getNextHint` increments index and fetches from backend; `handleRequestHint` appends or notifies when exhausted. 【F:src-ui/src/routes/+page.svelte†L279-L292】【F:src-ui/src/routes/+page.svelte†L76-L82】
  - Action submission: `sendAction` updates render state; `submitCode` and `submitQuestCode` call respective endpoints, update render state, reload quest on completion. 【F:src-ui/src/routes/+page.svelte†L294-L337】
  - UI overlays: renders GameWorld with HUD, CodeTerminal (quest-aware or single challenge), level-complete modal returning to map, toasts, progress tracker; Settings modal can load saved render state. 【F:src-ui/src/routes/+page.svelte†L339-L459】
- Backend selector (`src-ui/src/lib/backend/index.ts`)
  - Chooses Tauri → WASM → HTTP based on environment, caching the backend instance. 【F:src-ui/src/lib/backend/index.ts†L1-L48】
- HTTP backend (`src-ui/src/lib/backend/http.ts`)
  - Manages device ID, wraps REST endpoints for init/game actions/levels/code/quests/hints/progress/saves with fetch helper. Poller emits game-tick updates and supports event subscriptions while respecting visibility/pause. 【F:src-ui/src/lib/backend/http.ts†L1-L154】

## TODO / Intended Flow
- Backend player actions: `OpenInventory` should toggle/render inventory state in GameState for UI; `UseItem` should apply inventory effects and update progression/world accordingly. Currently stubs in action handler. 【F:src-api/src/main.rs†L418-L423】
- WASM bridge: helper comment suggests expanding type checks when bridging JS types. 【F:src-ui/src/lib/wasm/code_warrior_wasm.js†L161-L162】
- Door interaction placeholder: `interact_with_nearest` notes where door logic should live (e.g., unlock conditions, transitions). 【F:src/game/state.rs†L221-L233】
