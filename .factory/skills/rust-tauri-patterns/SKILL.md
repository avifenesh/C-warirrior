---
name: rust-tauri-patterns
description: Backend-authoritative Rust/Tauri implementation patterns for Code Warrior's game engine
---

# Rust/Tauri Patterns for Code Warrior

Expert in implementing backend-authoritative game features with Rust and Tauri.

## Core Architecture Pattern

```
Frontend (Svelte) → Tauri Command → Rust Logic → State Update → Tauri Event → Frontend Update
```

**Key Rule**: Frontend NEVER mutates authoritative state. Only displays it.

## Pattern 1: Adding a New Game Feature

### Step-by-Step Implementation

**1. Define Rust Data Structures**
```rust
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Player {
    pub position: Position,
    pub inventory: Vec<Item>,
    pub xp: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
```

**2. Implement Pure Logic Functions**
```rust
impl Player {
    pub fn move_to(&mut self, new_pos: Position) -> Result<(), String> {
        // Validation
        if !self.can_move_to(&new_pos) {
            return Err("Invalid position".to_string());
        }

        // State mutation
        self.position = new_pos;
        Ok(())
    }

    fn can_move_to(&self, pos: &Position) -> bool {
        // Pure logic - no side effects
        pos.x >= 0.0 && pos.y >= 0.0
    }
}
```

**3. Create Tauri Command**
```rust
use tauri::State;

#[tauri::command]
async fn move_player(
    x: f32,
    y: f32,
    state: State<'_, AppState>,
) -> Result<Player, String> {
    let mut game_state = state.game.lock().await;

    game_state.player.move_to(Position { x, y })?;

    // Return updated state
    Ok(game_state.player.clone())
}
```

**4. Emit Events for State Changes**
```rust
use tauri::Manager;

#[tauri::command]
async fn use_item(
    item_id: String,
    app: tauri::AppHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut game_state = state.game.lock().await;

    game_state.inventory.remove_item(&item_id)?;

    // Emit event to all listeners
    app.emit_all("inventory-updated", &game_state.inventory)
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

**5. Generate TypeScript Types**
```bash
# Add to build process
cargo test  # Generates bindings/*.ts files
```

**6. Create Svelte Component with Runes**
```svelte
<script lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { Player } from '$lib/types/bindings';

let player = $state<Player | null>(null);

// Listen for events
$effect(() => {
    const unlisten = listen('player-updated', (event) => {
        player = event.payload as Player;
    });

    return () => { unlisten.then(fn => fn()); };
});

async function movePlayer(x: number, y: number) {
    try {
        player = await invoke('move_player', { x, y });
    } catch (error) {
        console.error('Failed to move:', error);
    }
}
</script>
```

## Pattern 2: State Management

### Global Game State (Rust)
```rust
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct AppState {
    pub game: Arc<Mutex<GameState>>,
}

pub struct GameState {
    pub player: Player,
    pub world: World,
    pub inventory: Inventory,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            game: Arc::new(Mutex::new(GameState::default())),
        }
    }
}

// In main.rs
fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            move_player,
            use_item,
            // ... other commands
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Frontend State (Svelte - View Only)
```svelte
<script lang="ts">
// Reactive state derived from Rust events
let gameState = $state({
    player: null,
    inventory: [],
    world: null
});

// Listen to backend events
$effect(() => {
    listen('game-state-updated', (event) => {
        gameState = event.payload;
    });
});
</script>
```

## Pattern 3: Error Handling

### Rust Side
```rust
#[derive(Debug, Serialize)]
pub struct GameError {
    code: String,
    message: String,
}

impl From<GameError> for String {
    fn from(err: GameError) -> String {
        serde_json::to_string(&err).unwrap_or(err.message)
    }
}

#[tauri::command]
async fn risky_action(
    state: State<'_, AppState>,
) -> Result<Response, String> {
    let mut game = state.game.lock().await;

    game.do_something()
        .map_err(|e| GameError {
            code: "ACTION_FAILED".to_string(),
            message: e.to_string(),
        }.into())
}
```

### Frontend Side
```svelte
<script lang="ts">
async function doAction() {
    try {
        await invoke('risky_action');
    } catch (error) {
        const err = JSON.parse(error as string);
        showError(err.message);
    }
}
</script>
```

## Pattern 4: Async Operations

### Background Processing
```rust
use tokio::task;

#[tauri::command]
async fn compile_c_code(
    code: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // Emit progress events
    app.emit_all("compilation-started", ()).ok();

    let result = task::spawn_blocking(move || {
        // Blocking C compilation
        compile(&code)
    })
    .await
    .map_err(|e| e.to_string())?;

    app.emit_all("compilation-finished", &result).ok();

    Ok(result)
}
```

## Pattern 5: Testing Game Logic

### Unit Tests (Pure Logic)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_movement() {
        let mut player = Player::default();

        let result = player.move_to(Position { x: 10.0, y: 5.0 });
        assert!(result.is_ok());
        assert_eq!(player.position.x, 10.0);
    }

    #[test]
    fn test_invalid_movement() {
        let mut player = Player::default();

        let result = player.move_to(Position { x: -1.0, y: 0.0 });
        assert!(result.is_err());
    }
}
```

### Integration Tests (Commands)
```rust
#[cfg(test)]
mod command_tests {
    use super::*;

    #[tokio::test]
    async fn test_move_player_command() {
        let state = AppState::new();

        let result = move_player(10.0, 5.0, State::from(&state)).await;
        assert!(result.is_ok());
    }
}
```

## Pattern 6: Database Integration

### Using Diesel with Tauri
```rust
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub struct DbState {
    pub conn: Arc<Mutex<SqliteConnection>>,
}

#[tauri::command]
async fn save_progress(
    player: Player,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.conn.lock().await;

    diesel::insert_into(players::table)
        .values(&player)
        .execute(&*conn)
        .map_err(|e| e.to_string())?;

    Ok(())
}
```

## Common Anti-Patterns to Avoid

### ❌ Anti-Pattern: Logic in Frontend
```svelte
<!-- DON'T DO THIS -->
<script>
let xp = 100;
function gainXp(amount) {
    xp += amount * 1.5; // Game logic in frontend!
}
</script>
```

### ✅ Correct Pattern: Logic in Rust
```rust
// DO THIS
#[tauri::command]
async fn gain_xp(
    amount: u32,
    state: State<'_, AppState>,
) -> Result<u32, String> {
    let mut game = state.game.lock().await;
    game.player.xp += (amount as f32 * 1.5) as u32;
    Ok(game.player.xp)
}
```

### ❌ Anti-Pattern: Polling for State
```svelte
<!-- DON'T DO THIS -->
<script>
setInterval(async () => {
    player = await invoke('get_player'); // Wasteful!
}, 100);
</script>
```

### ✅ Correct Pattern: Event-Driven Updates
```svelte
<!-- DO THIS -->
<script>
listen('player-updated', (event) => {
    player = event.payload; // Efficient!
});
</script>
```

## File Organization

```
src/
├── main.rs                  # Tauri setup
├── commands/                # Tauri command handlers
│   ├── mod.rs
│   ├── player.rs
│   ├── inventory.rs
│   └── world.rs
├── game/                    # Pure game logic
│   ├── mod.rs
│   ├── player.rs
│   ├── physics.rs
│   └── xp.rs
├── types/                   # Shared types
│   ├── mod.rs
│   └── player.rs
└── db/                      # Database
    ├── mod.rs
    └── models.rs
```

## Performance Considerations

1. **Minimize Command Calls**: Batch operations when possible
2. **Use Events for Broadcasts**: One update → many listeners
3. **Clone Strategically**: Only clone data that leaves Rust
4. **Lock Duration**: Keep `Mutex` locks short-lived
5. **Blocking Operations**: Use `spawn_blocking` for CPU-intensive tasks

## Security Notes

- **Validate all frontend input** in Rust commands
- **Never trust frontend calculations** for game logic
- **Sanitize file paths** before file operations
- **Rate limit expensive operations**

This backend-authoritative pattern ensures game integrity while maintaining a responsive UI.
