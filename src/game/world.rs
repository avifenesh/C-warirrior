use serde::{Deserialize, Serialize};

use super::state::Position;
use crate::levels::WorldConfig;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TileType {
    Floor,
    Wall,
    Water,
    Void, // NULL/danger zone
    Door,
    Terminal, // Code submission point
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub tile_type: TileType,
    pub walkable: bool,
    pub interactable: bool,
    /// Quest ID for terminals (links terminal to specific quest)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quest_id: Option<String>,
}

impl Tile {
    pub fn floor() -> Self {
        Self {
            tile_type: TileType::Floor,
            walkable: true,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn wall() -> Self {
        Self {
            tile_type: TileType::Wall,
            walkable: false,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn terminal() -> Self {
        Self {
            tile_type: TileType::Terminal,
            walkable: true,
            interactable: true,
            quest_id: None,
        }
    }

    /// Create a terminal linked to a specific quest
    pub fn terminal_with_quest(quest_id: String) -> Self {
        Self {
            tile_type: TileType::Terminal,
            walkable: true,
            interactable: true,
            quest_id: Some(quest_id),
        }
    }

    pub fn door() -> Self {
        Self {
            tile_type: TileType::Door,
            walkable: false,
            interactable: true,
            quest_id: None,
        }
    }

    pub fn water() -> Self {
        Self {
            tile_type: TileType::Water,
            walkable: false,
            interactable: false,
            quest_id: None,
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
        let mut tiles = vec![vec![Tile::floor(); width]; height];

        // Add wall border around the map
        for x in 0..width {
            tiles[0][x] = Tile::wall(); // Top wall
            tiles[height - 1][x] = Tile::wall(); // Bottom wall
        }
        for y in 0..height {
            tiles[y][0] = Tile::wall(); // Left wall
            tiles[y][width - 1] = Tile::wall(); // Right wall
        }

        // Player spawns at tile (10, 7) = pixel (320, 240)
        let spawn_tile_x = 10;
        let spawn_tile_y = 7;
        let spawn_point = Position::new(
            (spawn_tile_x as f32 * 32.0) + 16.0, // Center of tile
            (spawn_tile_y as f32 * 32.0) + 16.0,
        );

        // Add terminal at tile (15, 7) - 5 tiles to the right of spawn
        if spawn_tile_y < height && 15 < width {
            tiles[spawn_tile_y][15] = Tile::terminal();
        }

        // Add locked door at tile (18, 7) - right edge (before wall at x=19)
        if spawn_tile_y < height && 18 < width {
            tiles[spawn_tile_y][18] = Tile::door();
        }

        // Add some decorative water patches
        if 5 < height && 5 < width {
            tiles[3][5] = Tile::water();
            tiles[3][6] = Tile::water();
            tiles[4][5] = Tile::water();
        }

        Self {
            width,
            height,
            tiles,
            spawn_point,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map(|t| t.walkable).unwrap_or(false)
    }

    /// Unlock a door at the specified tile coordinates
    pub fn unlock_door(&mut self, x: usize, y: usize) {
        if let Some(tile) = self.tiles.get_mut(y).and_then(|row| row.get_mut(x)) {
            if tile.tile_type == TileType::Door {
                tile.walkable = true;
            }
        }
    }

    /// Unlock all doors in the world
    pub fn unlock_all_doors(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                if tile.tile_type == TileType::Door {
                    tile.walkable = true;
                }
            }
        }
    }

    pub fn from_config(config: &WorldConfig) -> Self {
        let mut world = Self::new(config.width, config.height);
        world.spawn_point = Position::new(config.spawn_x, config.spawn_y);

        // Place terminals from the terminals array (preferred)
        if !config.terminals.is_empty() {
            for terminal in &config.terminals {
                let tx = (terminal.x / 32.0) as usize;
                let ty = (terminal.y / 32.0) as usize;
                if ty < config.height && tx < config.width {
                    if let Some(ref quest_id) = terminal.quest_id {
                        world.tiles[ty][tx] = Tile::terminal_with_quest(quest_id.clone());
                    } else {
                        world.tiles[ty][tx] = Tile::terminal();
                    }
                }
            }
        } else {
            // Legacy: single terminal at configured position
            let tx = (config.terminal_x / 32.0) as usize;
            let ty = (config.terminal_y / 32.0) as usize;
            if ty < config.height && tx < config.width {
                world.tiles[ty][tx] = Tile::terminal();
            }
        }

        world
    }

    /// Get the quest_id for a tile at given coordinates
    pub fn get_tile_quest_id(&self, x: usize, y: usize) -> Option<&str> {
        self.get_tile(x, y).and_then(|t| t.quest_id.as_deref())
    }
}
