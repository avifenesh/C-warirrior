# WASM Game Core Spec – Local GameState in Browser

Audience: smart coding agent implementing a WebAssembly (WASM) build of the Rust game core, so the web frontend gets “instant” movement while keeping all game logic in Rust.

## 1. Goal & Constraints

**Goal:** Run `GameState` and related logic locally in the browser via WASM, so player movement and other interactions feel immediate. The backend remains responsible for:
- C code compilation/execution and puzzle validation.
- Persistence (sessions, progress, saves).
- Serving levels and assets.

**Key constraints (from CONSTRAINTS.md):**
- All game logic must be in Rust. Frontend JS/Svelte is rendering + input only.
- No gameplay rules are re‑implemented in TypeScript; JS can only call into Rust/WASM.
- Tauri/desktop continues to use native Rust logic; WASM only applies to the browser/web build.

## 2. Current Architecture (Relevant Pieces)

Rust core (`src/`):
- `src/game/state.rs` – `GameState`, `GamePhase`, `PlayerAction`, `RenderState` and movement/interaction logic.
- `src/game/progression.rs` – `ProgressionState` and level unlock/XP logic.
- `src/levels/loader.rs` – `LevelData`, `LevelRegistry` loading from `src/assets/levels.json`.
- `src/lib.rs` – re‑exports `GameState`, `RenderState`, `LevelRegistry`, etc.

Backends:
- HTTP: `src-api/src/main.rs` with routes for `init`, `process_action`, `render-state`, `levels`, `submit_code`, progress, saves.
- Tauri: `src-tauri/src/main.rs` + `src-tauri/src/commands/*.rs` using `GameStateWrapper` and `LevelRegistry` directly.

Frontend:
- `src-ui/src/lib/backend/http.ts` – HTTP implementation of `Backend` interface.
- `src-ui/src/lib/backend/tauri.ts` – Tauri implementation of `Backend` interface.
- `src-ui/src/lib/backend/index.ts` – `getBackend()` picks HTTP vs Tauri.
- `src-ui/src/routes/+page.svelte` – binds `renderState` from backend to `GameWorld.svelte`.
- `src-ui/src/lib/components/GameWorld.svelte` – canvas rendering & input.

Right now, the web path always goes through HTTP for each action; movement waits for network + backend.

## 3. Target Architecture (High-Level)

For the **web** build:
- Add a WASM module that exposes a thin API around `GameState` and `LevelRegistry`:
  - Create/reset game.
  - Load a level by ID.
  - Apply `PlayerAction` (move, interact, pause/resume).
  - Get `RenderState` snapshot.
- Add a new `WasmBackend` implementation of the `Backend` interface that:
  - Uses the WASM module for game lifecycle, actions, levels, progress.
  - Uses the existing HTTP API only for:
    - `submitCode` (C compiler + validator),
    - persistence (`/api/saves*`, `/api/player/progress`),
    - possibly loading level JSON and assets.
- Keep Tauri and HTTP backends as they are for now; WASM only replaces web move/level/state flow.

This gives “instant” local movement but keeps backend as source of truth for puzzle correctness and stored progression.

## 4. Rust/WASM Module Design

### 4.1 New crate or feature

Approach A (recommended): new crate `code-warrior-wasm` in the workspace:
- Depends on `code_warrior` (the core library in `src/`).
- Built for `wasm32-unknown-unknown` with `wasm-bindgen`.

Approach B: feature flag on `code_warrior`:
- Add a `wasm` feature that pulls in `wasm-bindgen` only when targeting `wasm32`.
- Expose a `wasm` API module behind that feature.

**Tasks:**
- [ ] Add a `code-warrior-wasm` crate (or `wasm` feature) in `Cargo.toml` workspace.
- [ ] Ensure `code_warrior` core (`game`, `levels`) compiles for `wasm32-unknown-unknown` (no OS‑specific code in those modules).
  - `persistence` and filesystem can remain non‑WASM; they won’t be used on web.

### 4.2 WASM API Surface

Keep the JS‑visible API very small and JSON‑friendly:

```rust
// pseudo-code in code-warrior-wasm/src/lib.rs
use wasm_bindgen::prelude::*;
use code_warrior::{GameState, PlayerAction, RenderState, LevelRegistry};

#[wasm_bindgen]
pub struct WasmGame {
    state: GameState,
    levels: LevelRegistry,
}

#[wasm_bindgen]
impl WasmGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGame {
        WasmGame {
            state: GameState::default(),
            levels: LevelRegistry::load_from_json(),
        }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.state = GameState::default();
    }

    #[wasm_bindgen]
    pub fn load_level(&mut self, level_id: String) -> Result<JsValue, JsValue> {
        // same logic as HTTP/Tauri load_level, but without DB
        // return RenderState as JsValue (via serde_wasm_bindgen)
    }

    #[wasm_bindgen]
    pub fn process_action(&mut self, action_json: &JsValue) -> Result<JsValue, JsValue> {
        // Deserialize PlayerAction, apply to self.state, return RenderState
    }

    #[wasm_bindgen]
    pub fn get_render_state(&self) -> Result<JsValue, JsValue> {
        // Serialize RenderState to JsValue
    }
}
```

**Tasks:**
- [ ] Add `wasm-bindgen` and `serde_wasm_bindgen` (or similar) to the WASM crate.
- [ ] Define `WasmGame` wrapper exposing methods for: `new`, `reset`, `load_level`, `process_action`, `get_render_state`.
- [ ] Use the existing core functions (`GameState::move_player`, `interact_with_nearest`, `start_level`, `complete_level`) inside these methods; no new game rules.
- [ ] Ensure `RenderState`, `PlayerAction` are `Serialize`/`Deserialize` and JSON-friendly.

### 4.3 Submit Code & Progress in WASM

For code submission, you still need the backend C compiler. Two options:

- **Option 1 (simpler):** Backend remains the authority for puzzle correctness.
  - Frontend calls `/api/code/submit` with code.
  - Backend returns `CodeResult` including `success`, `xp_earned`, `render_state`.
  - WASM path:
    - Use backend `success` to decide whether to call a WASM method to mark the level complete (e.g. `apply_success()` that calls `GameState::complete_level` and updates progression).
    - Or trust backend and simply replace WASM `GameState` using the returned `RenderState` mapped back into Rust (more complex).

- **Option 2 (tight coupling):** Backend returns only `ExecutionOutput`, and WASM `LevelData::validate_output` is the single source of puzzle correctness. This requires more plumbing, but keeps all game logic (including validation) local in WASM.

**Tasks:**
- [ ] Decide which option to use (recommend Option 1 initially).
- [ ] Add WASM methods to apply a successful completion (e.g., `on_level_completed(xp_reward)`), mirroring backend `GameState::complete_level` logic.
- [ ] Ensure progress state (`ProgressionState`) can be exported to TS for syncing to backend (see persistence below).

## 5. JS/WASM Integration on Frontend

### 5.1 New `WasmBackend` implementation

Add a new backend implementation for web only:

```ts
// src-ui/src/lib/backend/wasm.ts (sketch)
import type { Backend, RenderState, GameState, PlayerAction, LevelData, LevelInfo, CodeResult, SaveSlot, PlayerProgress } from './types';
import initWasm, { WasmGame } from 'code-warrior-wasm'; // built & exposed via Vite config

export async function createWasmBackend(): Promise<Backend> {
  await initWasm();
  const game = new WasmGame();

  return {
    async initGame(): Promise<RenderState> {
      // reset game and return initial render state
      game.reset();
      return game.get_render_state();
    },
    async getGameState(): Promise<GameState> {
      // Optionally expose GameState; or rely on RenderState only
      throw new Error('getGameState not implemented for WASM backend');
    },
    async getRenderState(): Promise<RenderState> {
      return game.get_render_state();
    },
    async processAction(action: PlayerAction): Promise<RenderState> {
      return game.process_action(action);
    },
    async getAvailableLevels(): Promise<LevelInfo[]> {
      // From LevelRegistry via WASM or via HTTP `/api/levels`
    },
    async loadLevel(levelId: string): Promise<void> {
      await game.load_level(levelId);
    },
    // Code, Save/Load, Progress still call HTTP backend for now
    submitCode: (code: string) => httpBackend.submitCode(code),
    getHint: (idx: number) => httpBackend.getHint(idx),
    listSaves: () => httpBackend.listSaves(),
    saveGame: (slotId: string) => httpBackend.saveGame(slotId),
    loadGame: async (slotId: string) => {
      const render = await httpBackend.loadGame(slotId);
      // Optionally sync WASM state here later
      return render;
    },
    deleteSave: (slotId: string) => httpBackend.deleteSave(slotId),
    getProgress: () => httpBackend.getProgress(),
    onGameTick: httpBackend.onGameTick,
    onCodeOutput: httpBackend.onCodeOutput,
    onLevelComplete: httpBackend.onLevelComplete,
    onGameError: httpBackend.onGameError,
    cleanup() { /* noop or wasm cleanup */ },
  };
}
```

**Tasks:**
- [ ] Configure Vite/Tauri to build and load the WASM module (probably via `vite-plugin-wasm` or similar).
- [ ] Implement `createWasmBackend()` as above.
- [ ] In `src-ui/src/lib/backend/index.ts`, decide when to use WASM backend:
  - For web (non‑Tauri), prefer WASM.
  - Keep HTTP backend as an optional fallback (e.g., feature flag or environment variable).

### 5.2 Frontend Changes for Instant Movement

Once WASM backend is in place:
- `+page.svelte` and `GameWorld.svelte` do not need to change much: they still call `backend.processAction` and use `renderState` updates. Because `WasmBackend` is local, `processAction` resolves almost immediately, giving instant feedback.
- You can still apply client-side prediction if desired, but it may become unnecessary given WASM speed.

**Tasks:**
- [ ] Verify that `backend` interface is unchanged from the page’s perspective; only the implementation differs.
- [ ] Keep all game rules in WASM; do not add any new movement logic in JS just because WASM is available.

## 6. Persistence & Sync Considerations

Even with local WASM state, you still need to persist progress:
- **Sessions / progress:** continue to use HTTP `/api/player/progress` and existing DB tables. Periodically send `PlayerProgress` from WASM (serialized) so backend can save it.
- **Save/Load:** keep using the existing HTTP/Tauri save system. In a second phase, you can add WASM support to directly load/save `GameState` snapshots to/from backend responses.

**Tasks:**
- [ ] Add an easy way to export `PlayerProgress` from WASM (e.g., `get_progress()` returning a JSON blob) and use it in the web backend for syncing.
- [ ] Defer full WASM–backend state sync (e.g. loading server saves into WASM) to a later phase; initially, you can trust backend render_state after save/load and gradually move more state sync into WASM wrappers.

## 7. Risks & Re-Evaluation Points

- **Duplication of logic between WASM and HTTP/Tauri:**  
  For some time, you may have both server-side and client-side simulations. Make sure tests assert they stay in sync (see below).
- **Complexity in build pipeline:**  
  WASM introduces a new build target and bundling step. Ensure local build scripts/dev commands (Vite and Tauri) are updated and documented.
- **Security/cheating (low risk here):**  
  For an educational single-player game, trusting client-side movement and progression is acceptable. If you later add competitive features, you might need server-side verification.

**Re-eval recommendations (another set of eyes should focus on):**
- API surface: are the WASM exports minimal and stable? Avoid exposing low-level details that tie JS too closely to Rust internals.
- Sync strategy: is it acceptable that backend and WASM can diverge between sync points? If not, plan more frequent syncs or keep server simulation for certain checks.

## 8. Testing & Parity

To be confident in WASM correctness:
- Add a test harness (pure Rust) that:
  - Runs a fixed sequence of `PlayerAction`s through `GameState` compiled for native target and for `wasm32` (via wasm-bindgen test or host runner).
  - Compares resulting `RenderState` / `ProgressionState` snapshots.
- For web:
  - Write a small debug view that logs `RenderState` from WASM vs HTTP backend for the same seed input, just during development.

This spec should give a future agent all the relevant context and a clear, staged set of tasks to implement WASM‑based local game logic without violating the “Rust-only logic” constraint or breaking existing Tauri/HTTP flows.  Using WASM in this way is the most robust path to “press → move feels instant” on the web.

## 9. Post-Implementation Cleanup Checklist

Once the WASM backend is implemented, tested, and stable, do a focused cleanup pass to remove temporary paths and keep the codebase simple:

- **Backend selection & fallbacks**
  - [ ] If the WASM backend is the default and proven, decide whether HTTP polling for movement/render on web is still needed:
    - Keep HTTP `process_action` / `render-state` for diagnostics or non-WASM fallback; or
    - Gate them behind a feature flag, so they are not used in normal web builds.
  - [ ] Remove any temporary feature flags or env switches that are no longer necessary once you pick the final behavior.

- **Duplicate logic & dead paths**
  - [ ] Remove any leftover client-side prediction JS that is no longer used because WASM handles movement instantly.
  - [ ] Delete obsolete code paths in `http.ts` that were only there to support old polling flows (e.g., unused event stubs or legacy endpoints) if they are truly unused.
  - [ ] Clean up any unused types or helper functions in TS that were only needed during the migration (e.g., experimental action queues).

- **Docs & specs**
  - [ ] Update `docs/interfaces/tauri-commands.md` and any frontend API docs to reflect the final architecture (WASM for web, Tauri/native for desktop).
  - [ ] Mark older HTTP-based movement flows as legacy or remove them from docs if they are no longer supported.
  - [ ] Revisit `cleanup-*`, performance, and WASM specs and tick off completed items or prune sections that are now obsolete.

- **Debugging & logging**
  - [ ] Remove or downgrade any extra logging added while bringing up WASM (e.g., per-frame dumps, state comparison logs).
  - [ ] Keep only structured logs that are genuinely useful for support or future debugging.

- **Local tests**
  - [ ] Ensure there is a simple local command or script to build the WASM target and run at least a basic WASM smoke test in a browser or headless environment.
  - [ ] Remove temporary test harnesses that were only for manual comparison, or convert them into permanent regression tests that can be run locally before merging changes.

Doing this cleanup after WASM is stable will keep the project maintainable and avoid confusion between legacy HTTP movement, Tauri/native flows, and the new WASM-driven web path.
