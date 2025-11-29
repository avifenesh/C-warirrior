use crate::game::state::Position;
use crate::game::world::{Tile, TileType, World};
use serde::Deserialize;

// --- Tiled JSON Format ---

#[derive(Debug, Clone, Deserialize)]
pub struct TiledMap {
    pub width: usize,
    pub height: usize,
    #[serde(rename = "tilewidth")]
    pub tile_width: u32,
    #[serde(rename = "tileheight")]
    pub tile_height: u32,
    pub layers: Vec<TiledLayer>,
    #[serde(default)]
    pub properties: Vec<TiledProperty>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type")]
pub enum TiledLayer {
    #[serde(rename = "tilelayer")]
    TileLayer {
        name: String,
        data: Vec<u32>,
        width: usize,
        height: usize,
    },
    #[serde(rename = "objectgroup")]
    ObjectGroup {
        name: String,
        objects: Vec<TiledObject>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledObject {
    #[serde(default)]
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub object_type: String,
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    #[serde(default)]
    pub properties: Vec<TiledProperty>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledProperty {
    pub name: String,
    #[serde(rename = "type")]
    pub property_type: String,
    pub value: serde_json::Value,
}

// --- Legacy Custom Format ---

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<u32>>,
    pub spawn: LegacyPos,
    pub objects: Vec<LegacyObject>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyPos {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegacyObject {
    #[serde(rename = "type")]
    pub object_type: String,
    pub x: usize,
    pub y: usize,
    #[serde(default)]
    pub locked: bool,
}

// --- Unified Loader ---

#[derive(Debug, Clone)]
pub enum MapFormat {
    Tiled(TiledMap),
    Legacy(LegacyMap),
}

impl MapFormat {
    pub fn to_world(&self) -> World {
        match self {
            MapFormat::Tiled(map) => map.to_world(),
            MapFormat::Legacy(map) => map.to_world(),
        }
    }
}

impl TiledMap {
    pub fn to_world(&self) -> World {
        let mut world = World::new(self.width, self.height);

        for layer in &self.layers {
            match layer {
                TiledLayer::TileLayer {
                    name, data, width, ..
                } => {
                    if name == "floor" || name == "tiles" {
                        for (i, &tile_id) in data.iter().enumerate() {
                            let x = i % width;
                            let y = i / width;
                            if y < self.height && x < self.width {
                                world.tiles[y][x] = tile_id_to_tile(tile_id);
                            }
                        }
                    } else if name == "collision" {
                        for (i, &tile_id) in data.iter().enumerate() {
                            let x = i % width;
                            let y = i / width;
                            if y < self.height && x < self.width && tile_id != 0 {
                                // Assuming non-zero in collision layer means blocked
                                world.tiles[y][x].walkable = false;
                            }
                        }
                    }
                }
                TiledLayer::ObjectGroup { objects, .. } => {
                    for obj in objects {
                        // Tiled coordinates are in pixels, World expects tiles for grid placement
                        // But wait, World uses Position (float pixels) for spawn
                        // and Tile grid (indices) for tiles.

                        let grid_x = (obj.x / self.tile_width as f32) as usize;
                        let grid_y = (obj.y / self.tile_height as f32) as usize;

                        match obj.object_type.as_str() {
                            "spawn" => {
                                world.spawn_point = Position::new(obj.x, obj.y);
                            }
                            "terminal" => {
                                if grid_y < self.height && grid_x < self.width {
                                    world.tiles[grid_y][grid_x] = Tile::terminal();
                                }
                            }
                            "door" => {
                                if grid_y < self.height && grid_x < self.width {
                                    let mut is_locked = false;
                                    for prop in &obj.properties {
                                        if prop.name == "locked" {
                                            if let Some(val) = prop.value.as_bool() {
                                                is_locked = val;
                                            }
                                        }
                                    }

                                    world.tiles[grid_y][grid_x] = Tile {
                                        tile_type: TileType::Door,
                                        walkable: !is_locked,
                                        interactable: true,
                                    };
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        world
    }
}

impl LegacyMap {
    pub fn to_world(&self) -> World {
        let mut world = World::new(self.width, self.height);

        // Set tiles from 2D array
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile_id) in row.iter().enumerate() {
                if y < self.height && x < self.width {
                    world.tiles[y][x] = tile_id_to_tile(tile_id);
                }
            }
        }

        // Set spawn
        world.spawn_point = Position::new(
            (self.spawn.x as f32 * 32.0) + 16.0,
            (self.spawn.y as f32 * 32.0) + 16.0,
        );

        // Set objects
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
        0 => Tile {
            // Void
            tile_type: TileType::Void,
            walkable: false,
            interactable: false,
        },
        1 => Tile::floor(),
        2 => Tile::floor(), // Tech floor is still floor
        3 => Tile::wall(),
        4 => Tile::wall(), // Wall top is still wall
        5 => Tile::terminal(),
        6 => Tile::door(), // Locked door
        7 => Tile {
            // Open door
            tile_type: TileType::Door,
            walkable: true,
            interactable: true,
        },
        _ => Tile::floor(),
    }
}

pub fn load_map_file(map_path: &str) -> Result<MapFormat, String> {
    let json_str = match map_path {
        // Themed maps (new unique layouts)
        "maps/L01_first_spell.json" => include_str!("../assets/maps/L01_first_spell.json"),
        "maps/L02_empty_backpack.json" => include_str!("../assets/maps/L02_empty_backpack.json"),
        "maps/L03_gatekeeper.json" => include_str!("../assets/maps/L03_gatekeeper.json"),
        "maps/L04_forest.json" => include_str!("../assets/maps/L04_forest.json"),
        "maps/L05_darkwoods.json" => include_str!("../assets/maps/L05_darkwoods.json"),
        "maps/L06_river.json" => include_str!("../assets/maps/L06_river.json"),
        "maps/L07_cavern.json" => include_str!("../assets/maps/L07_cavern.json"),
        "maps/L08_mountain.json" => include_str!("../assets/maps/L08_mountain.json"),
        "maps/L09_ice.json" => include_str!("../assets/maps/L09_ice.json"),
        "maps/L10_temple.json" => include_str!("../assets/maps/L10_temple.json"),
        "maps/L11_library.json" => include_str!("../assets/maps/L11_library.json"),
        "maps/L12_crypt.json" => include_str!("../assets/maps/L12_crypt.json"),
        "maps/L13_lake.json" => include_str!("../assets/maps/L13_lake.json"),
        "maps/L14_forge.json" => include_str!("../assets/maps/L14_forge.json"),
        "maps/L15_lair.json" => include_str!("../assets/maps/L15_lair.json"),
        "maps/L16_courtyard.json" => include_str!("../assets/maps/L16_courtyard.json"),
        "maps/L17_throne.json" => include_str!("../assets/maps/L17_throne.json"),
        "maps/L18_treasury.json" => include_str!("../assets/maps/L18_treasury.json"),
        "maps/L19_dungeon.json" => include_str!("../assets/maps/L19_dungeon.json"),
        "maps/L20_passage.json" => include_str!("../assets/maps/L20_passage.json"),
        "maps/L21_stairs.json" => include_str!("../assets/maps/L21_stairs.json"),
        "maps/L22_alchemy.json" => include_str!("../assets/maps/L22_alchemy.json"),
        "maps/L23_observatory.json" => include_str!("../assets/maps/L23_observatory.json"),
        "maps/L24_portal.json" => include_str!("../assets/maps/L24_portal.json"),
        "maps/L25_sanctum.json" => include_str!("../assets/maps/L25_sanctum.json"),
        // Legacy fallbacks (old challenge-based maps)
        "maps/L04_repeating_strike.json" => include_str!("../assets/maps/L04_repeating_strike.json"),
        "maps/L05_array_fortress.json" => include_str!("../assets/maps/L05_array_fortress.json"),
        _ => return Err(format!("Unknown map file: {}", map_path)),
    };

    // Try parsing as TiledMap first
    if let Ok(tiled_map) = serde_json::from_str::<TiledMap>(json_str) {
        return Ok(MapFormat::Tiled(tiled_map));
    }

    // Fallback to LegacyMap
    match serde_json::from_str::<LegacyMap>(json_str) {
        Ok(legacy_map) => Ok(MapFormat::Legacy(legacy_map)),
        Err(e) => Err(format!(
            "Failed to parse map {} as either Tiled or Legacy: {}",
            map_path, e
        )),
    }
}
