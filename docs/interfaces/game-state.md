# Game State Interfaces

**Owner**: Opus 4.5 Reasoning (Phase 0)
**Consumers**: All agents

This document defines the core data structures for Code Warrior's game state.
Both Rust and TypeScript versions are provided for type safety across the IPC boundary.

---

## Core Types

### Position

```rust
// Rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn tile_coords(&self, tile_size: f32) -> (i32, i32) {
        ((self.x / tile_size) as i32, (self.y / tile_size) as i32)
    }
}
```

```typescript
// TypeScript
export interface Position {
    x: number;
    y: number;
}
```

---

### Player

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub position: Position,
    pub health: u32,
    pub max_health: u32,
    pub xp: u32,
    pub level: u32,
    pub facing: Direction,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            position: Position::new(0.0, 0.0),
            health: 100,
            max_health: 100,
            xp: 0,
            level: 1,
            facing: Direction::Down,
        }
    }
}
```

```typescript
// TypeScript
export type Direction = 'up' | 'down' | 'left' | 'right';

export interface Player {
    position: Position;
    health: number;
    max_health: number;
    xp: number;
    level: number;
    facing: Direction;
}
```

---

### Tile & World

```rust
// Rust
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Water,
    Void,        // NULL/danger zone
    Door,
    Terminal,    // Code submission point
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
    pub interactable: bool,
}

impl Tile {
    pub fn floor() -> Self {
        Self { tile_type: TileType::Floor, walkable: true, interactable: false }
    }

    pub fn wall() -> Self {
        Self { tile_type: TileType::Wall, walkable: false, interactable: false }
    }

    pub fn terminal() -> Self {
        Self { tile_type: TileType::Terminal, walkable: true, interactable: true }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Tile>>,
    pub spawn_point: Position,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Tile::floor(); width]; height];
        Self {
            width,
            height,
            tiles,
            spawn_point: Position::new(1.0, 1.0),
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map(|t| t.walkable).unwrap_or(false)
    }
}
```

```typescript
// TypeScript
export type TileType = 'floor' | 'wall' | 'water' | 'void' | 'door' | 'terminal';

export interface Tile {
    tile_type: TileType;
    walkable: boolean;
    interactable: boolean;
}

export interface World {
    width: number;
    height: number;
    tiles: Tile[][];
    spawn_point: Position;
}
```

---

### GameState (Main State Container)

```rust
// Rust - src/game/state.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub world: World,
    pub current_level_id: Option<String>,
    pub game_phase: GamePhase,
    pub progression: ProgressionState,
    /// Active quest ID when at a terminal (for multi-quest levels)
    pub active_quest_id: Option<String>,
    // Legacy fields for backwards compatibility
    #[serde(skip)]
    pub total_xp: u32,
    #[serde(skip)]
    pub levels_completed: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GamePhase {
    MainMenu,
    Playing,
    Coding,      // Player is at terminal, writing code
    Paused,
    LevelComplete,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player: Player::default(),
            world: World::new(20, 15),
            current_level_id: None,
            game_phase: GamePhase::MainMenu,
            progression: ProgressionState::new(),
            active_quest_id: None,
            total_xp: 0,
            levels_completed: Vec::new(),
        }
    }
}

impl GameState {
    /// Complete a quest and award XP
    pub fn complete_quest(&mut self, level_id: &str, quest_id: &str, xp_reward: u32) -> u32 {
        let xp_earned = self.progression.complete_quest(level_id, quest_id, xp_reward);
        self.player.xp += xp_earned;
        self.total_xp = self.progression.total_xp;
        xp_earned
    }

    /// Check if all quests in a level are completed
    pub fn is_level_fully_completed(&self, level_id: &str, total_quests: usize) -> bool {
        self.progression.is_level_fully_completed(level_id, total_quests)
    }
}
```

```typescript
// TypeScript - src-ui/src/lib/types.ts
export type GamePhase = 'main_menu' | 'playing' | 'coding' | 'paused' | 'level_complete';

export interface GameState {
    player: Player;
    world: World;
    current_level_id: string | null;
    game_phase: GamePhase;
    total_xp: number;
    levels_completed: string[];
}
```

---

### RenderState (Optimized for Frontend)

The `RenderState` is a subset of `GameState` sent to the frontend on each tick.
It contains only what's needed for rendering to minimize IPC overhead.

```rust
// Rust - src/game/state.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderState {
    pub player: Player,
    pub visible_tiles: Vec<Vec<Tile>>,
    pub viewport_offset: Position,
    pub game_phase: GamePhase,
    pub current_level_id: Option<String>,
    pub map: Option<TileMapRender>,
    pub objects: Vec<ObjectRender>,
    pub show_terminal: bool,
    pub active_dialogue: Option<String>,
    /// The quest ID of the terminal the player is interacting with
    pub active_quest_id: Option<String>,
}
```

```typescript
// TypeScript - src-ui/src/lib/types.ts
export interface RenderState {
    player: Player;
    visible_tiles: Tile[][];
    viewport_offset: Position;
    game_phase: GamePhase;
    current_level_id: string | null;
    map?: TileMapRender | null;
    objects: ObjectRender[];
    show_terminal: boolean;
    active_dialogue: string | null;
    /** The quest ID of the terminal the player is interacting with */
    active_quest_id: string | null;
}
```

---

### PlayerAction (Frontend to Backend)

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PlayerAction {
    Move { direction: Direction },
    Interact,
    SubmitCode { code: String },
    Pause,
    Resume,
}
```

```typescript
// TypeScript
export type PlayerAction =
    | { type: 'move'; direction: Direction }
    | { type: 'interact' }
    | { type: 'submit_code'; code: string }
    | { type: 'pause' }
    | { type: 'resume' };
```

---

## Constants

```rust
// Rust - src/game/constants.rs
pub const TILE_SIZE: f32 = 32.0;
pub const VIEWPORT_WIDTH: usize = 20;
pub const VIEWPORT_HEIGHT: usize = 15;
pub const PLAYER_SPEED: f32 = 200.0; // pixels per second
pub const TICK_RATE: u64 = 20; // ticks per second
pub const XP_PER_LEVEL: u32 = 100;
```

```typescript
// TypeScript - src-ui/src/lib/constants.ts
export const TILE_SIZE = 32;
export const VIEWPORT_WIDTH = 20;
export const VIEWPORT_HEIGHT = 15;
export const PLAYER_SPEED = 200;
export const TICK_RATE = 20;
export const XP_PER_LEVEL = 100;
```

---

## File Mapping

| Interface | Rust File | TypeScript File |
|-----------|-----------|-----------------|
| Position | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| Player | `src/game/player.rs` | `src-ui/src/lib/types.ts` |
| Tile, World | `src/game/world.rs` | `src-ui/src/lib/types.ts` |
| GameState | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| RenderState | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| PlayerAction | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| ProgressionState | `src/game/progression.rs` | N/A (internal) |
| Constants | `src/game/constants.rs` | N/A |

---

## Usage Notes

### For Development
- Import types from `$lib/types.ts` in the frontend
- Use `RenderState` for rendering, not full `GameState`
- `PlayerAction` is what you send via the backend abstraction
- Never mutate state directly, always send actions to backend

### For Type Synchronization
- Rust types are authoritative
- TypeScript types in `src-ui/src/lib/types.ts` must match Rust
- Serialize Rust enums as `snake_case` for TypeScript
