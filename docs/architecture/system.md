# Code Warrior: Technical Architecture

## For AI agents and developers
- Use this to understand the Rust/Tauri/Svelte architecture and backend‑authoritative model.
- When in doubt, put game logic and C execution in Rust, and keep Svelte as a visualization layer.
- Do not introduce patterns that contradict the command/event IPC model described here.
- Keep runtime flow in sync with [`docs/logic-mindmap.md`](../logic-mindmap.md); it is the source of truth for how Axum routes, game state helpers, and the Svelte page wire together in code.

## Table of Contents
1. [Architectural Overview](#architectural-overview)
2. [Technology Stack](#technology-stack)
3. [Backend-Authoritative Model](#backend-authoritative-model)
4. [Component Responsibilities](#component-responsibilities)
5. [IPC Communication Protocol](#ipc-communication-protocol)
6. [Game Loop Architecture](#game-loop-architecture)
7. [C Runtime Sandbox](#c-runtime-sandbox)
8. [State Management](#state-management)
9. [Performance Considerations](#performance-considerations)

---

## Architectural Overview

Code Warrior implements a **hybrid desktop application** architecture where:

- **Rust** acts as the authoritative game engine and systems layer
- **Svelte 5** provides reactive visualization and UI
- **Tauri 2.0** bridges the two as a desktop application framework

### Core Principle: Backend-Authoritative Pattern

This is NOT a traditional client-server web application. The Rust backend is the **single source of truth** for ALL game state. The Svelte frontend is a **pure visualization layer** with no business logic.

**Why this matters for AI agents:**
Standard web development patterns (React state management, frontend routing, client-side validation) DO NOT apply here. AI coding assistants must be explicitly constrained to prevent them from implementing game logic in JavaScript/TypeScript.

---

## Technology Stack

### Backend Stack (Rust)

| Component | Version | Purpose | Key Features |
|-----------|---------|---------|--------------|
| **Rust** | 2021+ | Core language | Memory safety, performance, concurrency |
| **Tauri** | 2.0 | Desktop framework | IPC bridge, window management, OS integration |
| **Axum** | 0.7+ | HTTP API | Web frontend communication |
| **Tokio** | 1.x | Async runtime | Game loop threading, async I/O |
| **SQLx** | 0.7+ | Database | Type-safe PostgreSQL operations |
| **PostgreSQL** | - | Persistence | Save states, progress tracking (Neon for production) |

### Frontend Stack (Svelte)

| Component | Version | Purpose | Key Features |
|-----------|---------|---------|--------------|
| **Svelte** | 5.0+ | UI framework | Fine-grained reactivity with Runes |
| **TailwindCSS** | 3.x | Styling | Utility-first styling |
| **TypeScript** | 5.x | Type safety | Interface contracts with Rust |
| **Canvas API** | Native | Rendering | Game world visualization |

### Development Tools

| Tool | Purpose |
|------|---------|
| **ts-rs** | Generate TypeScript types from Rust structs (optional) |
| **Tiled** | Level editor for handcrafted maps |
| **Playwright** | E2E testing for web frontend |

---

## Backend-Authoritative Model

### The Dungeon Master Pattern

The Rust process acts as a "Dungeon Master" that:
1. **Owns** all game state (player position, inventory, quest flags)
2. **Calculates** all game logic (physics, collisions, combat)
3. **Validates** all actions (can the player move here? do they have the key?)
4. **Executes** C code in a sandboxed environment
5. **Persists** state to SQLite transactionally

### Component Boundaries

```
┌─────────────────────────────────────────────────┐
│              SVELTE FRONTEND                     │
│  ┌─────────────────────────────────────┐        │
│  │  • Renders game world to Canvas     │        │
│  │  • Displays UI overlays             │        │
│  │  • Captures keyboard/mouse input    │        │
│  │  • Sends commands via Tauri IPC     │        │
│  │  • Listens for state update events  │        │
│  └─────────────────────────────────────┘        │
└─────────────┬───────────────────────────────────┘
              │ Tauri IPC Bridge
              │ (Commands ↓ / Events ↑)
┌─────────────┴───────────────────────────────────┐
│              RUST BACKEND                        │
│  ┌─────────────────────────────────────┐        │
│  │  Game Loop Thread                   │        │
│  │  ├─ Update physics                  │        │
│  │  ├─ Run AI/NPCs                     │        │
│  │  ├─ Check collisions                │        │
│  │  └─ Emit state changes              │        │
│  └─────────────────────────────────────┘        │
│  ┌─────────────────────────────────────┐        │
│  │  C Runtime Thread                   │        │
│  │  ├─ Compile user C code             │        │
│  │  ├─ Execute in sandbox              │        │
│  │  ├─ Capture stdout/stderr           │        │
│  │  └─ Enforce timeouts                │        │
│  └─────────────────────────────────────┘        │
│  ┌─────────────────────────────────────┐        │
│  │  Database Layer                     │        │
│  │  └─ SQLite + Diesel (Save/Load)    │        │
│  └─────────────────────────────────────┘        │
└─────────────────────────────────────────────────┘
```

---

## Component Responsibilities

### Rust Backend Responsibilities

| Domain | Rust Backend Handles | Examples |
|--------|---------------------|----------|
| **Physics** | Collision detection, valid move calculation | `can_move_to(x, y)` → `bool` |
| **Inventory** | Item management, slot logic, durability | `add_item(item_id)`, `use_item(slot)` |
| **World** | Terrain generation (WFC), tile storage | `generate_map() → Grid<Tile>` |
| **C Execution** | Compile, run, sandbox, timeout management | `run_c_code(source) → Output` |
| **Quests** | Progression flags, completion checks | `complete_quest(id)` |
| **Persistence** | Save/load via Diesel + SQLite | `save_game()`, `load_game(slot)` |

### Svelte Frontend Responsibilities

| Domain | Svelte Frontend Handles | Examples |
|--------|------------------------|----------|
| **Rendering** | Canvas drawing, sprite rendering, animations | `drawTile(x, y, sprite)` |
| **Input** | Keyboard/mouse capture, send to Rust | `onKeyPress → invoke('move', dir)` |
| **UI** | Health bars, inventory grid, dialogue boxes | Pure visual components |
| **Interpolation** | Smooth animations between game ticks | Lerp player position for 60fps |

### Critical Constraint for AI Agents

**NEVER implement in Svelte:**
- Game logic calculations (HP, damage, XP)
- Validation (can player afford this? do they have key?)
- State persistence
- C code execution

**ALWAYS implement in Svelte:**
- Visual effects and animations
- UI component structure
- Input event capture (then send to Rust)

---

## IPC Communication Protocol

### The Command-Event Pattern

Tauri provides two communication primitives:

#### 1. Commands (Frontend → Backend)

**Purpose**: Frontend requests an action from the backend.

**Pattern**: Async function calls that return results.

```rust
// Rust: Define command
#[tauri::command]
async fn move_player(direction: String) -> Result<bool, String> {
    // Validate move
    // Update state
    // Return success
    Ok(true)
}

// Register in main.rs
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![move_player])
```

```typescript
// TypeScript: Invoke command
import { invoke } from '@tauri-apps/api/core';

async function handleMove(dir: string) {
    const success = await invoke<boolean>('move_player', { direction: dir });
    if (!success) console.error('Invalid move');
}
```

#### 2. Events (Backend → Frontend)

**Purpose**: Backend pushes state updates to frontend.

**Pattern**: Pub/sub with typed payloads.

```rust
// Rust: Emit event
use tauri::Manager;

fn game_loop_tick(app: &AppHandle, state: &GameState) {
    app.emit_all("game_tick", GameTickPayload {
        player_pos: state.player.position,
        visible_entities: state.get_visible(),
    }).unwrap();
}
```

```typescript
// TypeScript: Listen for event
import { listen } from '@tauri-apps/api/event';

await listen<GameTickPayload>('game_tick', (event) => {
    gameState.playerPos = event.payload.player_pos;
    // Trigger Svelte reactivity
});
```

### Type Safety Contract

Use `ts-rs` to generate TypeScript interfaces from Rust structs:

```rust
use ts_rs::TS;

#[derive(Serialize, TS)]
#[ts(export, export_to = "../src/lib/types.ts")]
struct PlayerState {
    x: f32,
    y: f32,
    health: u32,
}
```

This generates `src/lib/types.ts`:
```typescript
export interface PlayerState {
    x: number;
    y: number;
    health: number;
}
```

### Communication Schema

| Command | Direction | Purpose | Returns |
|---------|-----------|---------|---------|
| `move_player` | FE → BE | Request movement | `bool` (success) |
| `submit_code` | FE → BE | Submit C code | `ExecutionResult` |
| `use_item` | FE → BE | Use inventory item | `ItemEffect` |
| `save_game` | FE → BE | Trigger save | `Result<(), Error>` |
| `load_game` | FE → BE | Load save slot | `GameState` |

| Event | Direction | Purpose | Payload |
|-------|-----------|---------|---------|
| `game_tick` | BE → FE | State update (20-60Hz) | `GameState` |
| `code_output` | BE → FE | C execution result | `ExecutionResult` |
| `quest_complete` | BE → FE | Achievement trigger | `QuestData` |

### HTTP (Axum) Contract Used by Web Frontend

The web build talks to the Axum API defined in `src-api/src/main.rs`; update this list when the flows in `docs/logic-mindmap.md` change.

| Endpoint | Method | Purpose | Notes |
|----------|--------|---------|-------|
| `/api/game/init` | POST | Create session, seed state | Bootstraps DB/migrations, caches session, then frontend fetches `/api/game/render-state`. |
| `/api/game/state` | GET | Full `GameState` snapshot | Used rarely; render path prefers `/render-state`. |
| `/api/game/render-state` | GET | Render-ready `RenderState` | Polling target for map view and play mode. |
| `/api/game/action` | POST | Apply `PlayerAction` | Routes movement, interaction, pause/resume, TODO: inventory actions. |
| `/api/levels` | GET | List `LevelInfo` | Merges registry with unlock/completion flags. |
| `/api/levels/{id}/load` | POST | Load level | Validates unlock, builds `World`, updates progression, returns `{ level_data, render_state }`. |
| `/api/code/submit` | POST | Compile/run C, validate | Completes level on success and returns feedback + render state. |
| `/api/code/submit-quest` | POST | Function-based quest submission | Runs per-test harness, updates XP, can complete level when all quests done. |
| `/api/levels/current/quests` | GET | Quest list for active level | Surfaces quest metadata + completion flags. |
| `/api/levels/current/quests/{quest_id}` | GET | Quest details | Fetch single quest by id for terminal view. |
| `/api/code/hint/{index}` | GET | Sequential hints | Streams hints in order for current quest/challenge. |
| `/api/player/progress` | GET | Player totals | Aggregates from `ProgressionState`. |
| `/api/saves` | GET | List all save slots | Returns a list of available save slots. |
| `/api/saves/{slot}` | POST | Create or update a save slot | Serializes `GameState` to the specified slot. |
| `/api/saves/{slot}` | GET | Load a specific save slot | Restores `GameState` from the specified slot. |
| `/api/saves/{slot}` | DELETE | Delete a save slot | Removes the specified save slot. |

Frontends that embed via Tauri continue to use command/event IPC for realtime ticks, but should mirror the same state transitions documented in the Axum routes.

---

## Game Loop Architecture

### Threading Model

Code Warrior uses a **multi-threaded** architecture to prevent UI blocking:

```
Main Thread (Tauri)
├─ Handles OS window events
├─ Processes IPC commands
└─ Delegates to worker threads

Game Loop Thread (Tokio)
├─ Fixed tick rate (20-60 TPS)
├─ Updates physics
├─ Runs NPC AI
└─ Emits state to frontend

C Runtime Thread (std::thread)
├─ Spawned per code execution
├─ Timeout enforced
└─ Isolated from main state
```

### Implementation Pattern

```rust
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::Manager;

pub fn start_game_loop(app: AppHandle, state: Arc<Mutex<GameState>>) {
    std::thread::spawn(move || {
        let tick_duration = Duration::from_millis(50); // 20 TPS
        let mut last_tick = Instant::now();

        loop {
            let now = Instant::now();
            let delta = now - last_tick;

            if delta >= tick_duration {
                // Lock state briefly
                let mut game_state = state.lock().unwrap();

                // Update game logic
                game_state.update_physics(delta);
                game_state.update_npcs(delta);
                game_state.check_collisions();

                // Emit to frontend
                app.emit_all("game_tick", game_state.get_render_state())
                    .unwrap();

                last_tick = now;
            }

            // Prevent busy-wait
            std::thread::sleep(Duration::from_millis(1));
        }
    });
}
```

### Tick Rate vs Frame Rate

- **Game Logic Tick Rate**: 20-30 TPS (server-like)
- **Render Frame Rate**: 60 FPS (Svelte/Canvas)
- **Frontend interpolates** between ticks for smooth animation

---

## C Runtime Sandbox

### Security Requirements

User-submitted C code is **untrusted** and must be:
1. **Isolated** from the game process
2. **Time-limited** to prevent infinite loops
3. **Resource-constrained** to prevent memory bombs

### Implementation Strategy

#### Option 1: Docker Container (Production)
```rust
async fn execute_c_code(source: &str) -> Result<Output, Error> {
    let container = Command::new("docker")
        .args(["run", "--rm", "--network=none", "--memory=128m",
               "gcc:alpine", "sh", "-c",
               &format!("echo '{}' | gcc -x c - -o /tmp/prog && timeout 2s /tmp/prog", source)])
        .output()
        .await?;

    Ok(Output {
        stdout: String::from_utf8_lossy(&container.stdout).to_string(),
        stderr: String::from_utf8_lossy(&container.stderr).to_string(),
    })
}
```

#### Option 2: Local GCC (Development)
```rust
use std::process::{Command, Stdio};
use tokio::time::timeout;

async fn execute_c_code_simple(source: &str) -> Result<Output, Error> {
    // Write to temp file
    let temp_file = "/tmp/user_code.c";
    std::fs::write(temp_file, source)?;

    // Compile
    let compile = Command::new("gcc")
        .args([temp_file, "-o", "/tmp/user_prog"])
        .output()?;

    if !compile.status.success() {
        return Err(Error::CompileError(String::from_utf8_lossy(&compile.stderr).to_string()));
    }

    // Execute with timeout
    let execution = timeout(
        Duration::from_secs(2),
        Command::new("/tmp/user_prog").output()
    ).await??;

    Ok(Output {
        stdout: String::from_utf8_lossy(&execution.stdout).to_string(),
        stderr: String::from_utf8_lossy(&execution.stderr).to_string(),
    })
}
```

### Validation Pipeline

```
User C Code
    ↓
Syntax Check (optional linting)
    ↓
Compilation (gcc/clang)
    ↓ (success)
Execution (timeout enforced)
    ↓
Output Capture
    ↓
Result Parsing
    ↓
Game Event Trigger
```

---

## State Management

### Rust State Architecture

```rust
pub struct GameState {
    pub player: Player,
    pub world: WorldMap,
    pub entities: Vec<Entity>,
    pub quests: QuestLog,
    pub inventory: Inventory,
}

impl GameState {
    pub fn update(&mut self, delta: Duration) {
        self.update_physics(delta);
        self.update_entities(delta);
        self.check_quest_conditions();
    }

    pub fn get_render_state(&self) -> RenderState {
        RenderState {
            player_pos: self.player.position,
            visible_tiles: self.world.get_visible_chunk(self.player.position),
            nearby_entities: self.get_nearby_entities(),
        }
    }
}
```

### Svelte State Architecture (Runes)

**IMPORTANT**: Use Svelte 5 Runes, NOT stores or `export let`.

```svelte
<script lang="ts">
import { listen } from '@tauri-apps/api/event';

// Reactive state using Runes
let gameState = $state({
    playerPos: { x: 0, y: 0 },
    health: 100,
    entities: []
});

// Derived computed values
let playerTile = $derived(
    Math.floor(gameState.playerPos.x / TILE_SIZE)
);

// Effect for event listening
$effect(() => {
    const unlisten = listen('game_tick', (event) => {
        gameState = event.payload; // Triggers reactivity
    });

    return () => unlisten.then(fn => fn());
});
</script>
```

### Persistence Layer

The API server uses SQLx with PostgreSQL (Neon in production):

```rust
// src-api/src/db.rs
use sqlx::{Pool, Postgres};

pub async fn init_database(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    // Run migrations to create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS game_sessions (
            device_id TEXT PRIMARY KEY,
            game_state JSONB NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    // Save slots table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS save_slots (
            device_id TEXT NOT NULL,
            slot_id TEXT NOT NULL,
            name TEXT NOT NULL,
            game_state JSONB NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            PRIMARY KEY (device_id, slot_id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}
```

Session management uses an in-memory cache with database fallback:

```rust
// AppState in src-api/src/main.rs
struct AppState {
    db: Pool<Postgres>,
    levels: Arc<LevelRegistry>,
    compiler: Arc<CCompiler>,
    sessions: DashMap<String, GameState>,  // In-memory cache
}
```

---

## Performance Considerations

### Optimization Strategies

| Area | Strategy | Implementation |
|------|----------|----------------|
| **IPC Overhead** | Send diffs, not full state | `StateDiff { changed_tiles: Vec<(x, y, tile)> }` |
| **Rendering** | Chunk-based culling | Only render visible 20x15 tile viewport |
| **Physics** | Spatial hashing | Grid-based collision detection |
| **Memory** | Object pooling | Reuse entity structs instead of alloc/dealloc |

### Profiling Points

- **Game Loop**: Target 20-30ms per tick max
- **IPC Events**: Batch small updates, emit at 20Hz not 60Hz
- **Canvas Rendering**: Use `requestAnimationFrame`, avoid layout thrashing
- **C Execution**: Timeout after 2s, kill zombie processes

### Bottleneck Mitigation

**Problem**: Game loop blocks on database writes.
**Solution**: Use async Diesel queries with Tokio, move saves to separate thread.

**Problem**: Too many IPC events flood the frontend.
**Solution**: Emit "game_tick" at 20Hz with full state, separate rare events (quest_complete) as individual emissions.

---

## References

- [Tauri IPC Documentation](https://tauri.app/v2/guides/inter-process-communication/)
- [Svelte 5 Runes](https://svelte.dev/docs/runes)
- [Diesel ORM Guide](https://diesel.rs/guides/getting-started)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

---

**Next**: See [GAME_DESIGN.md](../GAME_DESIGN.md) for C concept mappings and RPG mechanics.
