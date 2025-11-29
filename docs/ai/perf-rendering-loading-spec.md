# Rendering & Loading Performance Spec (Frontend-Focused)

Audience: smart coding agent implementing performance improvements for web + Tauri frontends.  
Scope: Svelte/TS and rendering engine under `src-ui/src/lib`, **without** changing core game rules in Rust.

## 1. Goal & Constraints

**Goal:** Reduce frame rendering cost and perceived loading time, especially on low/mid devices, while preserving visual quality and gameplay semantics.

**Key constraints (from AGENTS.md / CONSTRAINTS.md):**
- All game logic and authoritative state live in Rust (`src/`, `src-api`, `src-tauri`); frontend is rendering + input only.
- Do not change Rust APIs in this spec; if you need backend changes, they must go through a separate backend spec.
- Keep existing art pipeline and asset formats; changes should be about *how* we load and render, not asset content.

## 2. Current Architecture (Relevant Pieces)

**Frontend rendering pipeline:**
- `src-ui/src/routes/+page.svelte`
  - Owns high-level game state (`renderState`, `levels`, `playerProgress`).
  - Uses `backend` abstraction (`$lib/backend`) for HTTP/Tauri.
  - Passes `renderState`, flags, and callbacks into `GameWorld.svelte`.
- `src-ui/src/lib/components/GameWorld.svelte`
  - Handles keyboard input and dispatches `move` / `interact` events up.
  - On mount:
    - Creates `GameRenderer` with `<canvas>`.
    - Calls `loadAssets(DEFAULT_MANIFEST)` and sets `assets` in renderer.
    - Starts a `requestAnimationFrame` loop that:
      - Reads `renderState` from outer scope.
      - Updates camera, particles, animations.
      - Calls `renderer.render(state, particles, dt, currentTime, animState)`.
- `src-ui/src/lib/engine/renderer.ts`
  - `GameRenderer` holds canvas/context, `LoadedAssets`, and `RenderConfig`.
  - Each `render()`:
    - Clears canvas;
    - If `!state`: shows loading text;
    - Else draws tiles, player, highlights, particles every frame from scratch.
- `src-ui/src/lib/engine/assets.ts`
  - Defines `DEFAULT_MANIFEST` for sprites/tiles/audio.
  - `loadAssets(manifest)` loads all manifest assets with `Promise.all` (images + audio).
  - `preloadLevel()` currently just calls `loadAssets(DEFAULT_MANIFEST)` again.

**Backend update flow (for context):**
- HTTP: `src-ui/src/lib/backend/http.ts`
  - Polls `/api/game/render-state` every 500ms via `EventPoller` when a level is active.
  - Fetches snapshots for movement updates; also gets `render_state` in responses to `/api/code/submit` and `/api/saves/:slot`.
- Tauri: `src-ui/src/lib/backend/tauri.ts`
  - Listens to events like `game_tick` only if emitted on Rust side (currently not heavily used).

## 3. Key Performance Opportunities

### 3.1 Asset Loading & Reuse

Current issues:
- `GameWorld.svelte` calls `loadAssets(DEFAULT_MANIFEST)` inside `onMount`, reloading all assets on every mount.
- `preloadLevel()` does the same; no real per-level granularity or caching.

Impact:
- Unnecessary network and decode cost when returning to the game view or reloading.
- Longer initial time-to-interactive, especially on slower devices.

### 3.2 Render Loop & State Change Detection

Current issues:
- `GameWorld.svelte` runs a custom `requestAnimationFrame` loop that always calls `renderer.render()` regardless of whether anything has changed.
- The repo also has a generic `createGameLoop`/`createRenderLoop` in `gameloop.ts` that is not used here.

Impact:
- Redundant canvas draws when state is unchanged (e.g. idle main menu, paused game).
- Higher CPU/GPU usage than necessary.

### 3.3 Per-Frame Work in `GameRenderer`

Current issues:
- Every frame, `renderScene`:
  - Iterates over all visible tiles and recomputes tile color and sprite selection.
  - Recomputes distances for terminal highlights and door indicators per tile.
- No memoization of tile layers or static backgrounds.

Impact:
- O(N_tiles) work every frame, even when the camera and world are static.
- Avoidable work for large maps or slow integrated GPUs.

### 3.4 Network Polling Strategy (HTTP)

Current issues:
- HTTP backend uses a fixed 500ms polling interval while a level is active.
- Polling continues as long as `currentLevelData` is non-null, regardless of user activity beyond document visibility.

Impact:
- Extra network traffic and JSON parsing for idle sessions or players standing still.

## 4. Proposed Refactors & Tasks

Below are concrete tasks, with optional sketches. They’re grouped so another agent can implement in stages.

### 4.1 Asset Loader Singleton & Per-Level Preload

**Goal:** Load assets once, reuse across mounts, and make it cheap to add level-specific assets later.

**Sketch:**
```ts
// src-ui/src/lib/engine/assets-cache.ts
import { DEFAULT_MANIFEST, loadAssets, type LoadedAssets } from './assets';

let assetPromise: Promise<LoadedAssets> | null = null;

export function getGlobalAssets(): Promise<LoadedAssets> {
  if (!assetPromise) {
    assetPromise = loadAssets(DEFAULT_MANIFEST);
  }
  return assetPromise;
}
```

**Tasks:**
- [ ] Add a small `assets-cache` module exporting `getGlobalAssets()` as above.
- [ ] In `GameWorld.svelte`, replace direct `loadAssets(DEFAULT_MANIFEST)` call with `getGlobalAssets()`, and only call once per app lifetime.
- [ ] Make `preloadLevel(levelId)` optionally call `getGlobalAssets()` plus level-specific manifest (future extension) instead of reloading the same paths.
- [ ] Ensure error handling: if asset load fails once, set a flag and don’t keep retrying aggressively; show a clear error in UI.

**Re-eval:** Another agent should confirm this doesn’t conflict with future plans for per-level asset manifests (e.g., new backgrounds/tiles) and that the global cache fits memory budgets on low-tier machines.

### 4.2 Use Shared Game Loop Utility & Add Change Detection

**Goal:** Avoid redundant draws by standardizing the render loop and skipping frames when nothing changed.

**Sketch:**
```ts
// In GameWorld.svelte (pseudo)
import { createRenderLoop } from '$lib/engine/gameloop';

let lastRenderState: RenderState | null = null;

function shouldRender(next: RenderState | null): boolean {
  if (!next && !lastRenderState) return false;
  if (!next || !lastRenderState) return true;
  // Cheap checks: phase, current_level_id, player position, viewport offset length
  if (next.game_phase !== lastRenderState.game_phase) return true;
  if (next.current_level_id !== lastRenderState.current_level_id) return true;
  if (next.player.position.x !== lastRenderState.player.position.x ||
      next.player.position.y !== lastRenderState.player.position.y) return true;
  return false;
}

const loop = createRenderLoop(() => {
  const state = renderState;
  if (!shouldRender(state)) return;
  lastRenderState = state;
  renderer.render(state, particles, dt, performance.now(), animState);
});
```

**Tasks:**
- [ ] Replace the ad-hoc `requestAnimationFrame` loop in `GameWorld.svelte` with `createRenderLoop` (or `createGameLoop` if you want fixed-step updates).
- [ ] Implement a simple `shouldRender(prev, next)` function executing only cheap comparisons (do not deep-compare large tile arrays).
- [ ] Only call `renderer.render()` when `shouldRender` returns true.
- [ ] Ensure animation and particle updates still run; if particles should animate even when state is static, keep their update in the loop but allow a reduced render frequency when completely idle.

**Re-eval:** Another agent should evaluate whether animation/particles need continuous updates even when the player and tiles are static, and adjust `shouldRender` vs. update separation accordingly.

### 4.3 Tile Layer Caching in Renderer

**Goal:** Reduce per-frame tile drawing work by caching the static background layer.

**Sketch:**
```ts
// In GameRenderer
private tileCacheCanvas: HTMLCanvasElement | null = null;
private tileCacheDirty = true;

private rebuildTileCache(state: RenderState) {
  if (!this.assets) return;
  const { tileSize } = this.config;
  const width = state.visible_tiles[0]?.length ?? 0;
  const height = state.visible_tiles.length;

  if (!this.tileCacheCanvas) {
    this.tileCacheCanvas = document.createElement('canvas');
  }
  this.tileCacheCanvas.width = width * tileSize;
  this.tileCacheCanvas.height = height * tileSize;
  const ctx = this.tileCacheCanvas.getContext('2d')!;

  // Draw tiles once onto cache
  // (same loops as current tile drawing, but into offscreen canvas)
}

public render(state: RenderState | null, ...) {
  // ...
  if (this.tileCacheDirty) {
    this.rebuildTileCache(state);
    this.tileCacheDirty = false;
  }
  if (this.tileCacheCanvas) {
    this.ctx.drawImage(this.tileCacheCanvas, 0, 0);
  }
  // Then draw dynamic elements: doors if they change, player, particles, etc.
}
```

**Tasks:**
- [ ] Add an offscreen `tileCacheCanvas` to `GameRenderer` along with a `tileCacheDirty` flag.
- [ ] When a new level is loaded or `visible_tiles` grid size changes, mark the cache dirty from `render()` (cheap checks only, no deep tile comparisons).
- [ ] Implement `rebuildTileCache` that draws the entire background tiles layer once onto the offscreen canvas.
- [ ] Adjust `renderScene` to use the cached background first, then draw dynamic overlays (player, interactive highlights, particles), minimizing per-frame loops.
- [ ] Keep a fallback path (no caching) guarded by a debug flag if bugs arise.

**Re-eval:** Another agent should review this design for correctness with door state changes and potential future destructible terrain; if tiles become highly dynamic, we may need a more granular invalidation strategy (per-row or tile).

### 4.4 Smarter HTTP Polling

**Goal:** Reduce network calls and JSON parsing when the player is idle or the game is in non-active phases.

**Sketch:**
```ts
// In http.ts EventPoller usage
let lastActivity = Date.now();

function markActivity() {
  lastActivity = Date.now();
}

// In processAction / submitCode / loadLevel wrappers, call markActivity().

this.poller.subscribe(
  'game-tick',
  cb,
  500,
  () => {
    const active = this.currentLevelData !== null;
    const recentlyActive = Date.now() - lastActivity < 5_000;
    return active && recentlyActive;
  },
  () => this.getRenderState()
);
```

**Tasks:**
- [ ] Track a simple `lastActivity` timestamp in `HttpBackend` (movement, interact, code submit, level load).
- [ ] Modify `onGameTick` subscription to only poll aggressively when:
  - A level is loaded; and
  - The user has performed an action within a short window (e.g. 5–10 seconds); otherwise slow down or pause polling.
- [ ] Optionally: add a slower polling mode (e.g. every 5s) for long-term idle sessions to keep state fresh without constant traffic.

**Re-eval:** Another agent should verify this doesn’t interfere with UX expectations (e.g., UI still reflects code state after long idles) and perhaps consider exposing the idle/window durations as constants in `constants.ts`.

## 5. Recommendations for Second Review

Before implementation:
- Validate that **asset caching** memory usage is acceptable for target machines (especially browser tab memory in low-RAM environments).
- Decide whether **tile caching** must support per-tile dynamic changes beyond doors/terminals; if yes, design invalidation rules up-front.
- Confirm that **polling backoff** doesn’t conflict with any future plans for real-time WebSocket/SSE updates on HTTP.

After implementation:
- Profile a few representative devices (devtools performance + network tabs) while:
  - Loading into the main menu.
  - Entering first levels.
  - Walking around for 30–60 seconds.
  - Leaving the game idle for several minutes.
- Compare FPS, CPU usage, and network request counts before/after where possible.

## 6. Context Summary for Implementing Agent

When editing, keep these boundaries in mind:
- **Do not** move game rules or authoritative state to the frontend; all refactors must work purely at the display and client-control layer.
- **Do** reuse existing abstractions (`getBackend`, `GameRenderer`, `EventPoller`) instead of introducing parallel systems.
- **Prefer** small, incremental changes (e.g., introduce caching behind flags) over large rewrites; the goal is performance, not a new engine.

If further backend changes (Rust/HTTP/Tauri) seem necessary for performance (e.g., server-side throttling, compressed render snapshots), capture them in a separate backend performance spec rather than mixing them into this frontend-focused document.  

This spec should give another smart coding agent enough focused context to design and implement the improvements, with clear spots flagged for re-evaluation and validation.*** End Patch ***!
