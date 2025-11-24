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

### Item & Inventory

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub description: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Key,
    Weapon,
    Consumable,
    QuestItem,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub max_slots: usize,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        Self {
            items: Vec::new(),
            max_slots,
        }
    }

    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() < self.max_slots {
            // Check if item already exists (stack)
            if let Some(existing) = self.items.iter_mut().find(|i| i.id == item.id) {
                existing.quantity += item.quantity;
            } else {
                self.items.push(item);
            }
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_id: &str) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|i| i.id == item_id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
}
```

```typescript
// TypeScript
export type ItemType = 'key' | 'weapon' | 'consumable' | 'quest_item';

export interface Item {
    id: string;
    name: string;
    item_type: ItemType;
    description: string;
    quantity: number;
}

export interface Inventory {
    items: Item[];
    max_slots: number;
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
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub player: Player,
    pub world: World,
    pub inventory: Inventory,
    pub current_level_id: Option<String>,
    pub game_phase: GamePhase,
    pub total_xp: u32,
    pub levels_completed: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
            inventory: Inventory::new(10),
            current_level_id: None,
            game_phase: GamePhase::MainMenu,
            total_xp: 0,
            levels_completed: Vec::new(),
        }
    }
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_level(&mut self, level_id: String, world: World) {
        self.current_level_id = Some(level_id);
        self.world = world;
        self.player.position = self.world.spawn_point;
        self.game_phase = GamePhase::Playing;
    }

    pub fn complete_level(&mut self, xp_reward: u32) {
        if let Some(ref level_id) = self.current_level_id {
            if !self.levels_completed.contains(level_id) {
                self.levels_completed.push(level_id.clone());
            }
        }
        self.total_xp += xp_reward;
        self.player.xp += xp_reward;
        self.game_phase = GamePhase::LevelComplete;
    }

    pub fn enter_coding_mode(&mut self) {
        self.game_phase = GamePhase::Coding;
    }

    pub fn exit_coding_mode(&mut self) {
        self.game_phase = GamePhase::Playing;
    }
}
```

```typescript
// TypeScript
export type GamePhase = 'main_menu' | 'playing' | 'coding' | 'paused' | 'level_complete';

export interface GameState {
    player: Player;
    world: World;
    inventory: Inventory;
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
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderState {
    pub player: Player,
    pub visible_tiles: Vec<Vec<Tile>>,
    pub viewport_offset: Position,
    pub game_phase: GamePhase,
    pub current_level_id: Option<String>,
}

impl GameState {
    /// Generate render state for frontend (20x15 viewport centered on player)
    pub fn to_render_state(&self) -> RenderState {
        let viewport_width = 20;
        let viewport_height = 15;

        // Calculate viewport offset (center on player)
        let (px, py) = self.player.position.tile_coords(32.0);
        let offset_x = (px - (viewport_width as i32 / 2)).max(0) as usize;
        let offset_y = (py - (viewport_height as i32 / 2)).max(0) as usize;

        // Extract visible tiles
        let mut visible_tiles = Vec::new();
        for y in offset_y..(offset_y + viewport_height).min(self.world.height) {
            let row: Vec<Tile> = self.world.tiles[y]
                .iter()
                .skip(offset_x)
                .take(viewport_width)
                .cloned()
                .collect();
            visible_tiles.push(row);
        }

        RenderState {
            player: self.player.clone(),
            visible_tiles,
            viewport_offset: Position::new(offset_x as f32, offset_y as f32),
            game_phase: self.game_phase,
            current_level_id: self.current_level_id.clone(),
        }
    }
}
```

```typescript
// TypeScript
export interface RenderState {
    player: Player;
    visible_tiles: Tile[][];
    viewport_offset: Position;
    game_phase: GamePhase;
    current_level_id: string | null;
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
    OpenInventory,
    UseItem { item_id: String },
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
    | { type: 'open_inventory' }
    | { type: 'use_item'; item_id: string }
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
| Item, Inventory | `src/game/inventory.rs` | `src-ui/src/lib/types.ts` |
| Tile, World | `src/game/world.rs` | `src-ui/src/lib/types.ts` |
| GameState | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| RenderState | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| PlayerAction | `src/game/state.rs` | `src-ui/src/lib/types.ts` |
| Constants | `src/game/constants.rs` | `src-ui/src/lib/constants.ts` |

---

## Usage Notes

### For Sonnet 4.5 1M (Rust Backend)
- Implement all structs exactly as shown
- Add `#[derive(Debug, Clone, Serialize, Deserialize)]` to all types
- Use `Position` for all coordinate values
- `GameState` is the single source of truth

### For GPT 5.1 Codex Max (Svelte Frontend)
- Import types from `$lib/types.ts`
- Use `RenderState` for rendering, not full `GameState`
- `PlayerAction` is what you send via Tauri commands
- Never mutate state directly, always send actions to backend

### For Opus 4.5 Standard (Integration)
- Generate TypeScript types from Rust using ts-rs (optional)
- Or manually keep types in sync as defined here
- Serialize Rust enums as `snake_case` for TypeScript
