# Code Warrior Logic Mind Map (Backend + Frontend)

> Format: nested bullet hierarchy for easy parsing by AI coding agents. Each node lists the source of truth for the logic.

## Backend (Rust)
- Axum API (`src-api/src/main.rs`)
  - App state boot: connect Postgres, run migrations, load levels, create compiler, prime in-memory session cache, set CORS, start server. 【F:src-api/src/main.rs†L1-L121】
  - Session resolution: `get_or_create_session` pulls from in-memory cache, otherwise loads/creates DB record; `persist_session` writes back to DB and caches, also stores progress snapshot. 【F:src-api/src/main.rs†L240-L306】
  - Game init/render/state fetch: `/api/game/init` initializes GameState, `/api/game/state` returns raw state, `/api/game/render-state` returns render projection. 【F:src-api/src/main.rs†L308-L382】
  - Player action pipeline: `/api/game/action` deserializes `PlayerAction` and routes Move/Interact/Pause/Resume (SubmitCode is rejected with guidance to use the code endpoint); updated state cached for subsequent calls. 【F:src-api/src/main.rs†L409-L444】
  - Level listing/loading: `/api/levels` merges registry data with unlock/completion flags from `GameState`; `/api/levels/:id/load` validates unlock status, constructs `World` from level config, updates progression, persists session. 【F:src-api/src/main.rs†L432-L522】
  - Code submission (single challenge): `/api/code/submit` compiles/runs user code, validates output, completes level if successful, persists, returns feedback plus render state. 【F:src-api/src/main.rs†L522-L610】
  - Code submission (function/quest challenges): `run_function_based_challenge` executes generated harness per test case, awards XP, unlocks doors, persists state; `/api/code/submit-quest` mirrors this per quest and tracks quest completion, potentially completing the level when all quests done. 【F:src-api/src/main.rs†L612-L869】【F:src-api/src/main.rs†L896-L1016】
  - Quest/hint APIs: `/api/levels/current/quests` and `/api/levels/current/quests/:quest_id` surface quest metadata + completion flags; `/api/code/hint/:index` streams hints in order. 【F:src-api/src/main.rs†L870-L950】【F:src-api/src/main.rs†L1018-L1048】
  - Progress & saves: `/api/player/progress` derives totals from `ProgressionState`; save slots list/upsert/load using serialized `GameState`; delete endpoint removes slots. 【F:src-api/src/main.rs†L1050-L1160】
- Game core (`src/game/state.rs`)
  - GameState structure: tracks player/world/progression/active quest and phase; default spawns a 20×15 world. 【F:src/game/state.rs†L39-L76】
  - Level lifecycle: `start_level` sets world + player spawn and enters Playing; `complete_level` awards XP, syncs legacy totals, unlocks doors, flips to LevelComplete; `update_unlocked_levels`/`is_level_unlocked`/`is_level_completed` delegate to progression. 【F:src/game/state.rs†L78-L122】
  - Quest lifecycle: `complete_quest`, `is_quest_completed`, `get_completed_quest_count`, `is_level_fully_completed`, `maybe_complete_level` handle per-quest XP and mark level complete when all quests done. 【F:src/game/state.rs†L123-L167】
  - Mode switches: `enter_coding_mode` / `exit_coding_mode` gate movement vs terminal interaction. 【F:src/game/state.rs†L169-L175】
  - Movement & collision: `move_player` checks phase, computes displacement, rejects blocked paths via physics, updates facing. 【F:src/game/state.rs†L177-L197】
  - Interaction flow: `interact_with_nearest` searches nearby interactables; terminals set `active_quest_id` and enter coding; doors are stubbed for future logic. 【F:src/game/state.rs†L204-L238】
  - Rendering pipeline: `to_render_state` builds 20×15 viewport around player, returns tiles, offsets, phase flags, active quest for frontend. 【F:src/game/state.rs†L259-L302】
- World layout (`src/game/world.rs`)
  - Tile taxonomy & helpers for floor/wall/terminal/water plus quest-aware terminals. 【F:src/game/world.rs†L7-L55】
  - Default map seeds walls/borders, spawn, terminal, and decorative water. 【F:src/game/world.rs†L57-L107】
  - Config loader `from_config` constructs worlds from level JSON including quest-linked terminals and legacy terminal positions; `get_tile_quest_id` resolves quest triggers for terminals. 【F:src/game/world.rs†L136-L189】

## Frontend (Svelte)
- App shell (`src-ui/src/routes/+page.svelte`)
  - State scaffolding: tracks backend handle, render state, level data, code draft, UI status, hints, quest metadata, and screen mode (boot/map/playing). 【F:src-ui/src/routes/+page.svelte†L16-L66】
  - Quest-reactive effects: when terminal opens and backend exposes `active_quest_id`, auto-loads quest details, resets hints/output; closes clearing quest. 【F:src-ui/src/routes/+page.svelte†L84-L115】
  - Navigation: handlers for new/continue/start first level, selecting from map, returning to map, and computing next level ID for progress tracker. 【F:src-ui/src/routes/+page.svelte†L116-L190】
  - Input dispatch: move/interact events send `PlayerAction` to backend; terminal close resumes play. 【F:src-ui/src/routes/+page.svelte†L146-L153】【F:src-ui/src/routes/+page.svelte†L186-L190】
  - Boot sequence: on mount, acquire backend (WASM with HTTP fallback), init game, fetch levels + progress, bind event polling, then show world map; errors surface in UI banner. 【F:src-ui/src/routes/+page.svelte†L191-L247】
  - Level loading: `startLevel` posts to backend, fetches level data + render state, resets hints/output. 【F:src-ui/src/routes/+page.svelte†L256-L277】
  - Hint pipeline: `getNextHint` increments index and fetches from backend; `handleRequestHint` appends or notifies when exhausted. 【F:src-ui/src/routes/+page.svelte†L279-L292】【F:src-ui/src/routes/+page.svelte†L76-L82】
  - Action submission: `sendAction` updates render state; `submitCode` and `submitQuestCode` call respective endpoints, update render state, reload quest on completion. 【F:src-ui/src/routes/+page.svelte†L294-L337】
  - UI overlays: renders GameWorld with HUD, CodeTerminal (quest-aware or single challenge), level-complete modal returning to map, toasts, progress tracker; Settings modal can load saved render state. 【F:src-ui/src/routes/+page.svelte†L339-L459】
- Backend selector (`src-ui/src/lib/backend/index.ts`)
  - Chooses WASM → HTTP fallback based on availability, caching the backend instance. 【F:src-ui/src/lib/backend/index.ts†L1-L44】
- HTTP backend (`src-ui/src/lib/backend/http.ts`)
  - Manages device ID, wraps REST endpoints for init/game actions/levels/code/quests/hints/progress/saves with fetch helper. Poller emits game-tick updates and supports event subscriptions while respecting visibility/pause. 【F:src-ui/src/lib/backend/http.ts†L1-L154】

## TODO / Intended Flow
- Door interaction placeholder: `interact_with_nearest` notes where door logic could live (unlock conditions, transitions). 【F:src/game/state.rs†L204-L238】
- WASM bridge: helper comment suggests expanding type checks when bridging JS types. 【F:src-ui/src/lib/wasm/code_warrior_wasm.js†L161-L162】

---

## Implementation Status (Audit: 2025-12-02)

### WORKING Features
| Feature | Evidence | Location |
|---------|----------|----------|
| Player movement & collision | `move_player` uses physics checks and blocks walls before updating position | 【F:src/game/state.rs†L177-L197】【F:src/game/physics.rs†L4-L70】
| Terminal interaction | Nearest terminal enters coding mode and sets `active_quest_id` | 【F:src/game/state.rs†L204-L238】
| Quest-based progression | Completing quests/levels awards XP and can auto-complete levels when all quests are done | 【F:src/game/state.rs†L123-L167】
| Level load/progression | `/api/levels/:id/load` builds a world, updates unlocks, caches session | 【F:src-api/src/main.rs†L432-L522】
| Code submission | `/api/code/submit` and `/api/code/submit-quest` compile/run code and update progression | 【F:src-api/src/main.rs†L522-L1016】
| Save/Load (web) | Save slots persisted to Postgres via SQLx | 【F:src-api/src/main.rs†L1050-L1160】【F:src-api/src/db/operations.rs†L130-L231】
| Viewport rendering | `to_render_state` extracts a 20×15 window around the player | 【F:src/game/state.rs†L259-L302】
| Level data | 25 levels, each with 2–3 quests parsed at startup | 【F:src/assets/levels.json†L1-L900】

### STUB / MISSING Features
| Feature | Status | Location |
|---------|--------|----------|
| Door interaction | Decorative only; no unlock/transition logic | 【F:src/game/state.rs†L204-L238】
| Game tick loop | `GameState::update` is a placeholder; no periodic tick thread in API/WASM | 【F:src/game/state.rs†L240-L257】

### Known Issues
1. Door tiles are decorative only; level completion advances progression without door interaction.

---

## Theme System (Visual Assets)

### Architecture
Each level has a unique visual theme with themed tiles, NPCs, and terminals.

| Component | Location | Purpose |
|-----------|----------|---------|
| Theme configs | `src-ui/src/lib/config/themes.ts` | 25 theme definitions with descriptions |
| Asset loader | `src-ui/src/lib/engine/assets.ts` | `loadThemeTiles()` loads per-level assets |
| Renderer | `src-ui/src/lib/engine/renderer.ts` | Uses theme sprites for tiles/terminals |
| Theme assets | `src-ui/static/tiles/themes/{theme}/` | PNG sprites per theme |

### Theme Asset Structure
```
src-ui/static/tiles/themes/
├── L01_village/
│   ├── floor.png, floor_alt.png    # Ground tiles
│   ├── wall.png, wall_top.png      # Wall tiles
│   ├── decoration_1.png, decoration_2.png  # Props
│   ├── npc.png                     # Quest giver character (32x32)
│   └── terminal.png                # Quest pedestal/altar (32x32)
├── L02_market/
│   └── ... (same structure)
└── L25_sanctum/
    └── ...
```

### ThemeConfig Interface (`themes.ts`)
```typescript
interface ThemeConfig {
    id: string;           // "L01_village"
    name: string;         // "Starter Village"
    description: string;  // Theme description
    floorDesc: string;    // "worn cobblestone path"
    wallDesc: string;     // "wooden fence with vines"
    decoration1: string;  // "barrel"
    decoration2: string;  // "wooden signpost"
    colorPalette: string; // "warm browns, soft greens"
    npcDesc: string;      // "village elder with staff"
    terminalDesc: string; // "wooden notice board"
}
```

### Theme Loading Flow
1. Level loads → `renderer.setTheme(theme)` called
2. `loadThemeTiles()` fetches 8 sprites from `/tiles/themes/{theme}/`
3. Renderer caches theme in `themeTilesetCache`
4. `getTileSprite()` returns theme-specific sprites with fallback to defaults

### 25 Themed NPCs and Terminals (Generated 2024-11-30)
| Level | Theme | NPC | Terminal |
|-------|-------|-----|----------|
| L01 | Village | Village elder | Notice board |
| L02 | Market | Merchant | Market stall |
| L03 | Tower | Tower guard | Stone pedestal |
| L04 | Forest | Forest spirit | Mushroom altar |
| L05 | Darkwoods | Hooded ranger | Corrupted stump |
| L06 | River | Fisherman | Dock post |
| L07 | Cavern | Cave hermit | Crystal pedestal |
| L08 | Mountain | Climber | Stone cairn |
| L09 | Ice | Frost mage | Ice pedestal |
| L10 | Temple | Monk | Stone altar |
| L11 | Library | Scholar ghost | Magic lectern |
| L12 | Crypt | Gravekeeper | Tombstone |
| L13 | Lake | Water spirit | Lily shrine |
| L14 | Forge | Blacksmith | Anvil |
| L15 | Lair | Treasure hunter | Gold pile |
| L16 | Courtyard | Gardener | Fountain |
| L17 | Throne | Herald | Gold throne |
| L18 | Treasury | Treasurer | Treasure chest |
| L19 | Dungeon | Guard | Iron cage |
| L20 | Passage | Spy | Lever |
| L21 | Stairs | Watchman | Stone marker |
| L22 | Alchemy | Alchemist | Cauldron |
| L23 | Observatory | Stargazer | Orrery |
| L24 | Portal | Guardian | Portal frame |
| L25 | Sanctum | Archmage | Power crystal |

### Sprite Generation
- **Tool**: `tools/generate-theme-sprites.py`
- **API**: Gemini 2.0 Flash Image Generation
- **Output**: 32x32 PNG pixel art sprites
- **Total**: 50 sprites (25 NPCs + 25 terminals)
