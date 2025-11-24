use serde::Deserialize;
use crate::game::state::Position;
use crate::game::world::{Tile, TileType, World};

#[derive(Debug, Clone, Deserialize)]
pub struct MapFileData {
    pub name: String,
    pub width: usize,
    pub height: usize,
    #[serde(rename = "tileSize")]
    pub tile_size: u32,
    pub layers: Vec<MapLayer>,
    pub objects: Vec<MapObjectDef>,
    #[serde(default)]
    pub metadata: Option<MapMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapLayer {
    pub name: String,
    pub data: Vec<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapObjectDef {
    #[serde(rename = "type")]
    pub object_type: String,
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub locked: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MapMetadata {
    pub concept: Option<String>,
    pub hint: Option<String>,
}

impl MapFileData {
    pub fn to_world(&self) -> World {
        let mut world = World::new(self.width, self.height);
        
        // Find spawn point
        for obj in &self.objects {
            if obj.object_type == "spawn" {
                world.spawn_point = Position::new(
                    obj.x as f32 * self.tile_size as f32,
                    obj.y as f32 * self.tile_size as f32,
                );
                break;
            }
        }

        // Process floor layer for tile types
        if let Some(floor_layer) = self.layers.iter().find(|l| l.name == "floor") {
            for (i, &tile_id) in floor_layer.data.iter().enumerate() {
                let x = i % self.width;
                let y = i / self.width;
                if y < self.height && x < self.width {
                    world.tiles[y][x] = tile_id_to_tile(tile_id);
                }
            }
        }

        // Process collision layer to override walkability
        if let Some(collision_layer) = self.layers.iter().find(|l| l.name == "collision") {
            for (i, &collision) in collision_layer.data.iter().enumerate() {
                let x = i % self.width;
                let y = i / self.width;
                if y < self.height && x < self.width {
                    world.tiles[y][x].walkable = collision == 0;
                }
            }
        }

        // Place objects (terminals, doors)
        for obj in &self.objects {
            if obj.y < self.height && obj.x < self.width {
                match obj.object_type.as_str() {
                    "terminal" => {
                        world.tiles[obj.y][obj.x] = Tile::terminal();
                    }
                    "door" => {
                        world.tiles[obj.y][obj.x] = Tile {
                            tile_type: TileType::Door,
                            walkable: !obj.locked,
                            interactable: true,
                        };
                    }
                    _ => {}
                }
            }
        }

        world
    }
}

fn tile_id_to_tile(id: u32) -> Tile {
    match id {
        1 => Tile::floor(),
        2 => Tile::terminal(),
        3 => Tile::wall(),
        4 => Tile {
            tile_type: TileType::Void,
            walkable: false,
            interactable: false,
        },
        5 => Tile {
            tile_type: TileType::Door,
            walkable: false,
            interactable: true,
        },
        _ => Tile::floor(),
    }
}

pub fn load_map_file(map_path: &str) -> Result<MapFileData, String> {
    // For compile-time inclusion, we use a match on known map files
    let json_str = match map_path {
        "maps/L01_first_spell.json" => include_str!("../assets/maps/L01_first_spell.json"),
        "maps/L02_empty_backpack.json" => include_str!("../assets/maps/L02_empty_backpack.json"),
        "maps/L03_gatekeeper.json" => include_str!("../assets/maps/L03_gatekeeper.json"),
        "maps/L04_repeating_strike.json" => include_str!("../assets/maps/L04_repeating_strike.json"),
        _ => return Err(format!("Unknown map file: {}", map_path)),
    };

    serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse map {}: {}", map_path, e))
}
