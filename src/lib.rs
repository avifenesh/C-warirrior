// Code Warrior - C Programming Educational Game
// Rust backend library

pub mod compiler;
pub mod game;
pub mod levels;
pub mod models;

// Re-export commonly used types at the crate root
pub use game::{
    Direction, GamePhase, GameState, Inventory, Item, ItemType, Player, PlayerAction, Position,
    RenderState, Tile, TileType, World, PLAYER_SPEED, TICK_RATE, TILE_SIZE, VIEWPORT_HEIGHT,
    VIEWPORT_WIDTH, XP_PER_LEVEL,
};

pub use compiler::ExecutionOutput;
pub use levels::{LevelData, LevelInfo, LevelRegistry, SuccessCriteria, WorldConfig, WorldPreset};
