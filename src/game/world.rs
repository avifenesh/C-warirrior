use serde::{Deserialize, Serialize};

use super::state::Position;
use crate::levels::WorldConfig;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
    Water,
    Void,     // NULL/danger zone
    Door,
    Terminal, // Code submission point
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
    pub interactable: bool,
}

impl Tile {
    pub fn floor() -> Self {
        Self {
            tile_type: TileType::Floor,
            walkable: true,
            interactable: false,
        }
    }

    pub fn wall() -> Self {
        Self {
            tile_type: TileType::Wall,
            walkable: false,
            interactable: false,
        }
    }

    pub fn terminal() -> Self {
        Self {
            tile_type: TileType::Terminal,
            walkable: true,
            interactable: true,
        }
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

    pub fn from_config(config: &WorldConfig) -> Self {
        let mut world = Self::new(config.width, config.height);
        world.spawn_point = Position::new(config.spawn_x, config.spawn_y);

        // Place terminal at configured position
        let tx = (config.terminal_x / 32.0) as usize;
        let ty = (config.terminal_y / 32.0) as usize;
        if ty < config.height && tx < config.width {
            world.tiles[ty][tx] = Tile::terminal();
        }

        world
    }
}
