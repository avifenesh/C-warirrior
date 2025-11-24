use serde::{Deserialize, Serialize};

use super::state::Position;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub position: Position,
    pub health: u32,
    pub max_health: u32,
    pub xp: u32,
    pub level: u32,
    pub facing: Direction,
}

impl Default for Player {
    fn default() -> Self {
        // Start at center of default 20x15 map (tile 10, 7) * TILE_SIZE(32)
        Self {
            position: Position::new(320.0, 240.0),
            health: 100,
            max_health: 100,
            xp: 0,
            level: 1,
            facing: Direction::Down,
        }
    }
}
