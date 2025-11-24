// Game module - core game logic and state management
// All game types and logic should go here

pub mod constants;
pub mod inventory;
pub mod map;
pub mod physics;
pub mod player;
pub mod state;
pub mod world;

// Re-export commonly used types for convenience
pub use constants::*;
pub use inventory::{Inventory, Item, ItemType};
pub use map::{MapObject, ObjectRender, ObjectType, TileMap, TileMapRender};
pub use player::{Direction, Player};
pub use state::{GamePhase, GameState, PlayerAction, Position, RenderState};
pub use world::{Tile, TileType, World};
