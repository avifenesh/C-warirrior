use serde::{Deserialize, Serialize};

use super::state::Position;
use crate::levels::WorldConfig;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TileType {
    Floor,
    Wall,
    Water,
    Void,     // NULL/danger zone
    Door,
    Terminal, // Code submission point
    // Environmental tiles for atmospheric levels
    Tree,     // Forest obstacle (walkable=false)
    Rock,     // Rocky obstacle (walkable=false)
    Lava,     // Fire/danger zone (walkable=false)
    Ice,      // Frozen ground (walkable=true)
    Bridge,   // Cross over water (walkable=true)
    Grass,    // Decorative (walkable=true)
    Path,     // Decorative trail (walkable=true)
    Pit,      // Dark hole/void (walkable=false)
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

    // Environmental tile constructors
    pub fn tree() -> Self {
        Self {
            tile_type: TileType::Tree,
            walkable: false,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn rock() -> Self {
        Self {
            tile_type: TileType::Rock,
            walkable: false,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn lava() -> Self {
        Self {
            tile_type: TileType::Lava,
            walkable: false,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn ice() -> Self {
        Self {
            tile_type: TileType::Ice,
            walkable: true,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn bridge() -> Self {
        Self {
            tile_type: TileType::Bridge,
            walkable: true,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn grass() -> Self {
        Self {
            tile_type: TileType::Grass,
            walkable: true,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn path() -> Self {
        Self {
            tile_type: TileType::Path,
            walkable: true,
            interactable: false,
            quest_id: None,
        }
    }

    pub fn pit() -> Self {
        Self {
            tile_type: TileType::Pit,
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

    pub fn from_config(config: &WorldConfig) -> Self {
        let width = config.width;
        let height = config.height;

        // Start with empty floor grid (NOT the hardcoded template)
        let mut tiles = vec![vec![Tile::floor(); width]; height];

        // Add wall border
        for x in 0..width {
            tiles[0][x] = Tile::wall();
            tiles[height - 1][x] = Tile::wall();
        }
        for y in 0..height {
            tiles[y][0] = Tile::wall();
            tiles[y][width - 1] = Tile::wall();
        }

        // Parse tiles array from config (water, walls, doors, etc.)
        if let Some(ref tile_configs) = config.tiles {
            for tile_config in tile_configs {
                let x = tile_config.x;
                let y = tile_config.y;
                if y < height && x < width {
                    match tile_config.tile_type.as_str() {
                        "water" => tiles[y][x] = Tile::water(),
                        "wall" => tiles[y][x] = Tile::wall(),
                        "door" => tiles[y][x] = Tile::door(),
                        "floor" => tiles[y][x] = Tile::floor(),
                        // Environmental tiles
                        "tree" => tiles[y][x] = Tile::tree(),
                        "rock" => tiles[y][x] = Tile::rock(),
                        "lava" => tiles[y][x] = Tile::lava(),
                        "ice" => tiles[y][x] = Tile::ice(),
                        "bridge" => tiles[y][x] = Tile::bridge(),
                        "grass" => tiles[y][x] = Tile::grass(),
                        "path" => tiles[y][x] = Tile::path(),
                        "pit" => tiles[y][x] = Tile::pit(),
                        _ => {}
                    }
                }
            }
        }

        // Place terminals from config
        for terminal in &config.terminals {
            let tx = (terminal.x / 32.0) as usize;
            let ty = (terminal.y / 32.0) as usize;
            if ty < height && tx < width {
                if let Some(ref quest_id) = terminal.quest_id {
                    tiles[ty][tx] = Tile::terminal_with_quest(quest_id.clone());
                } else {
                    tiles[ty][tx] = Tile::terminal();
                }
            }
        }

        let spawn_point = Position::new(config.spawn_x, config.spawn_y);

        Self {
            width,
            height,
            tiles,
            spawn_point,
        }
    }

    /// Get the quest_id for a tile at given coordinates
    pub fn get_tile_quest_id(&self, x: usize, y: usize) -> Option<&str> {
        self.get_tile(x, y).and_then(|t| t.quest_id.as_deref())
    }

    /// Unlock all doors in the world (called when level is completed)
    pub fn unlock_all_doors(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                if tile.tile_type == TileType::Door {
                    tile.walkable = true;
                }
            }
        }
    }
}
