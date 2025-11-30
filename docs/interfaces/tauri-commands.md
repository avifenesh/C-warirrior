# Tauri Commands & Events

**Owner**: Opus 4.5 Standard (Integration)
**Consumers**: All agents

This document defines all Tauri IPC commands and events for Code Warrior.
Commands go Frontend -> Backend. Events go Backend -> Frontend.

---

## Commands (Frontend to Backend)

### Game Control Commands

#### `init_game`
Initialize a new game session.

```rust
// Rust
#[tauri::command]
pub async fn init_game(state: State<'_, GameStateWrapper>) -> Result<GameState, String> {
    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;
    *game_state = GameState::default();
    Ok(game_state.clone())
}
```

```typescript
// TypeScript
import { invoke } from '@tauri-apps/api/core';

export async function initGame(): Promise<GameState> {
    return await invoke<GameState>('init_game');
}
```

---

#### `get_game_state`
Fetch current game state (for initial load or sync).

```rust
#[tauri::command]
pub async fn get_game_state(state: State<'_, GameStateWrapper>) -> Result<GameState, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    Ok(game_state.clone())
}
```

```typescript
export async function getGameState(): Promise<GameState> {
    return await invoke<GameState>('get_game_state');
}
```

---

#### `process_action`
Process a player action and return updated state.

```rust
#[tauri::command]
pub async fn process_action(
    action: PlayerAction,
    state: State<'_, GameStateWrapper>,
) -> Result<RenderState, String> {
    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;

    match action {
        PlayerAction::Move { direction } => {
            game_state.move_player(direction);
        }
        PlayerAction::Interact => {
            game_state.interact_with_nearest();
        }
        PlayerAction::SubmitCode { code } => {
            // Handled by separate command
            return Err("Use submit_code command for code submission".to_string());
        }
        PlayerAction::Pause => {
            game_state.game_phase = GamePhase::Paused;
        }
        PlayerAction::Resume => {
            game_state.game_phase = GamePhase::Playing;
        }
        _ => {}
    }

    Ok(game_state.to_render_state())
}
```

```typescript
export async function processAction(action: PlayerAction): Promise<RenderState> {
    return await invoke<RenderState>('process_action', { action });
}
```

---

### Level Commands

#### `get_available_levels`
Get list of all available levels.

```rust
#[tauri::command]
pub async fn get_available_levels(
    levels: State<'_, LevelRegistry>,
) -> Result<Vec<LevelInfo>, String> {
    Ok(levels.get_all_info())
}

// LevelInfo is a simplified struct for the level select screen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelInfo {
    pub id: String,
    pub title: String,
    pub concept: String,
    pub completed: bool,
    pub locked: bool,
}
```

```typescript
export interface LevelInfo {
    id: string;
    title: string;
    concept: string;
    completed: boolean;
    locked: boolean;
}

export async function getAvailableLevels(): Promise<LevelInfo[]> {
    return await invoke<LevelInfo[]>('get_available_levels');
}
```

---

#### `load_level`
Load a specific level by ID.

```rust
#[tauri::command]
pub async fn load_level(
    level_id: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<LevelData, String> {
    let level = levels.get_level(&level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    let mut game_state = state.0.lock().map_err(|e| e.to_string())?;
    game_state.start_level(level_id, level.create_world());

    Ok(level.to_level_data())
}
```

```typescript
export async function loadLevel(levelId: string): Promise<LevelData> {
    return await invoke<LevelData>('load_level', { levelId });
}
```

---

#### `get_level_data`
Get data for the current level.

```rust
#[tauri::command]
pub async fn get_level_data(
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<LevelData, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    let level_id = game_state.current_level_id.as_ref()
        .ok_or("No level currently loaded")?;

    let level = levels.get_level(level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    Ok(level.to_level_data())
}
```

```typescript
export async function getLevelData(): Promise<LevelData> {
    return await invoke<LevelData>('get_level_data');
}
```

---

### Code Execution Commands

#### `submit_code`
Submit C code for compilation and execution.

```rust
#[tauri::command]
pub async fn submit_code(
    code: String,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
    compiler: State<'_, CCompiler>,
) -> Result<CodeResult, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    let level_id = game_state.current_level_id.as_ref()
        .ok_or("No level currently loaded")?;

    let level = levels.get_level(level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    // Compile and run
    let execution_result = compiler.compile_and_run(&code).await?;

    // Validate against success criteria
    let success = level.validate_output(&execution_result);

    Ok(CodeResult {
        success,
        stdout: execution_result.stdout,
        stderr: execution_result.stderr,
        compile_error: execution_result.compile_error,
        execution_time_ms: execution_result.execution_time_ms,
    })
}
```

```typescript
export async function submitCode(code: string): Promise<CodeResult> {
    return await invoke<CodeResult>('submit_code', { code });
}
```

---

#### `get_hint`
Get a hint for the current level.

```rust
#[tauri::command]
pub async fn get_hint(
    hint_index: usize,
    state: State<'_, GameStateWrapper>,
    levels: State<'_, LevelRegistry>,
) -> Result<String, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;
    let level_id = game_state.current_level_id.as_ref()
        .ok_or("No level currently loaded")?;

    let level = levels.get_level(level_id)
        .ok_or_else(|| format!("Level {} not found", level_id))?;

    level.get_hint(hint_index)
        .ok_or_else(|| "No more hints available".to_string())
}
```

```typescript
export async function getHint(hintIndex: number): Promise<string> {
    return await invoke<string>('get_hint', { hintIndex });
}
```

---

### Progress Commands

#### `get_progress`
Fetch aggregate progression information for the current player/session.

```rust
#[derive(Debug, Clone, Serialize)]
pub struct ProgressInfo {
    pub total_xp: u32,
    pub completed_levels: Vec<String>,
    pub current_level: Option<String>,
}

#[tauri::command]
pub async fn get_progress(state: State<'_, GameStateWrapper>) -> Result<ProgressInfo, String> {
    let game_state = state.0.lock().map_err(|e| e.to_string())?;

    Ok(ProgressInfo {
        total_xp: game_state.progression.total_xp,
        completed_levels: game_state
            .progression
            .completed_levels
            .iter()
            .cloned()
            .collect(),
        current_level: game_state.current_level_id.clone(),
    })
}
```

```typescript
export interface PlayerProgress {
    total_xp: number;
    completed_levels: string[];
    current_level: string | null;
}

export async function getProgress(): Promise<PlayerProgress> {
    return await invoke<PlayerProgress>('get_progress');
}
```

---

## Events (Backend to Frontend)

### `game_tick`
Emitted at 20Hz with current render state.

```rust
// Rust - Emit in game loop
use tauri::Manager;

fn game_loop_tick(app: &AppHandle, state: &GameState) {
    let render_state = state.to_render_state();
    app.emit("game_tick", render_state).unwrap();
}
```

```typescript
// TypeScript - Listen
import { listen } from '@tauri-apps/api/event';

export async function onGameTick(callback: (state: RenderState) => void): Promise<() => void> {
    const unlisten = await listen<RenderState>('game_tick', (event) => {
        callback(event.payload);
    });
    return unlisten;
}
```

---

### `code_output`
Emitted when C code produces output (streaming for long-running code).

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeOutput {
    pub stream: String,  // "stdout" or "stderr"
    pub content: String,
    pub is_final: bool,
}

fn emit_code_output(app: &AppHandle, stream: &str, content: &str, is_final: bool) {
    app.emit("code_output", CodeOutput {
        stream: stream.to_string(),
        content: content.to_string(),
        is_final,
    }).unwrap();
}
```

```typescript
export interface CodeOutput {
    stream: 'stdout' | 'stderr';
    content: string;
    is_final: boolean;
}

export async function onCodeOutput(callback: (output: CodeOutput) => void): Promise<() => void> {
    const unlisten = await listen<CodeOutput>('code_output', (event) => {
        callback(event.payload);
    });
    return unlisten;
}
```

---

### `level_complete`
Emitted when player successfully completes a level.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelCompleteEvent {
    pub level_id: String,
    pub xp_earned: u32,
    pub next_level_id: Option<String>,
}

fn emit_level_complete(app: &AppHandle, level_id: &str, xp_earned: u32, next_level_id: Option<String>) {
    app.emit("level_complete", LevelCompleteEvent {
        level_id: level_id.to_string(),
        xp_earned,
        next_level_id,
    }).unwrap();
}
```

```typescript
export interface LevelCompleteEvent {
    level_id: string;
    xp_earned: number;
    next_level_id: string | null;
}

export async function onLevelComplete(callback: (event: LevelCompleteEvent) => void): Promise<() => void> {
    const unlisten = await listen<LevelCompleteEvent>('level_complete', (event) => {
        callback(event.payload);
    });
    return unlisten;
}
```

---

### `game_error`
Emitted when an error occurs in the backend.

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameError {
    pub code: String,
    pub message: String,
    pub recoverable: bool,
}

fn emit_error(app: &AppHandle, code: &str, message: &str, recoverable: bool) {
    app.emit("game_error", GameError {
        code: code.to_string(),
        message: message.to_string(),
        recoverable,
    }).unwrap();
}
```

```typescript
export interface GameError {
    code: string;
    message: string;
    recoverable: boolean;
}

export async function onGameError(callback: (error: GameError) => void): Promise<() => void> {
    const unlisten = await listen<GameError>('game_error', (event) => {
        callback(event.payload);
    });
    return unlisten;
}
```

---

## Command Registration

All commands must be registered in `src/main.rs`:

```rust
// src/main.rs
use tauri::Manager;

mod commands;
mod game;
mod levels;
mod compiler;

use commands::{game::*, levels::*};
use game::state::GameState;
use std::sync::Mutex;

pub struct GameStateWrapper(pub Mutex<GameState>);

fn main() {
    tauri::Builder::default()
        .manage(GameStateWrapper(Mutex::new(GameState::default())))
        .manage(LevelRegistry::load_from_json())
        .manage(CCompiler::new())
        .invoke_handler(tauri::generate_handler![
            // Game commands
            init_game,
            get_game_state,
            process_action,
            // Level commands
            get_available_levels,
            load_level,
            get_level_data,
            // Code commands
            submit_code,
            get_hint,
            // Progress commands
            complete_level,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Frontend Backend Abstraction

On the frontend, use the backend abstraction in `src-ui/src/lib/backend` instead of a single `api.ts` file. It selects the correct implementation (Tauri or HTTP) at runtime:

```typescript
import { getBackend } from '$lib/backend';
import type { Backend } from '$lib/backend';

let backend: Backend | null = null;

async function boot() {
    backend = await getBackend();
    const renderState = await backend.initGame();
    // ...
}
```

---

## File Mapping

| Command/Event | Rust File | TypeScript File |
|---------------|-----------|-----------------|
| Game state & types | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| Tauri commands | `src-tauri/src/commands/` | `src-ui/src/lib/backend/tauri.ts` |
| HTTP routes | `src-api/src/main.rs` | `src-ui/src/lib/backend/http.ts` |
| Level registry | `src/levels/` | N/A (backend only) |
| Event structs | `src/events.rs` | `src-ui/src/lib/types.ts` |

---

## Usage Notes

### For Development
- Use the backend abstraction in `src-ui/src/lib/backend`
- The abstraction auto-detects Tauri vs HTTP environment
- Subscribe to events using the backend's `onGameTick()` etc.

### For Adding New Endpoints
- **Tauri**: Add command in `src-tauri/src/commands/`, register in `main.rs`
- **HTTP**: Add route handler in `src-api/src/main.rs`
- **Frontend**: Add method to `Backend` interface and both implementations
