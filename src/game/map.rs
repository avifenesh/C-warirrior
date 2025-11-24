use serde::{Deserialize, Serialize};

use super::state::Position;
use super::world::{Tile, TileType};

/// Represents a complete tile map for a level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<Tile>>,
    pub objects: Vec<MapObject>,
}

impl TileMap {
    pub fn new(width: u32, height: u32) -> Self {
        let tiles = vec![vec![Tile::floor(); width as usize]; height as usize];
        Self {
            width,
            height,
            tiles,
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: MapObject) {
        self.objects.push(object);
    }

    pub fn get_object_at(&self, position: Position) -> Option<&MapObject> {
        self.objects
            .iter()
            .find(|obj| obj.position.x == position.x && obj.position.y == position.y)
    }

    pub fn remove_object_at(&mut self, position: Position) -> Option<MapObject> {
        if let Some(index) = self
            .objects
            .iter()
            .position(|obj| obj.position.x == position.x && obj.position.y == position.y)
        {
            Some(self.objects.remove(index))
        } else {
            None
        }
    }
}

/// Represents an object placed on the map (terminals, doors, NPCs, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapObject {
    pub object_type: ObjectType,
    pub position: Position,
    pub interaction_id: Option<String>,
}

impl MapObject {
    pub fn new(object_type: ObjectType, position: Position) -> Self {
        Self {
            object_type,
            position,
            interaction_id: None,
        }
    }

    pub fn with_interaction_id(mut self, id: String) -> Self {
        self.interaction_id = Some(id);
        self
    }
}

/// Types of objects that can exist on the map
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Terminal,
    Door,
    Npc,
    Collectible,
}

/// Simplified render version of a map object for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectRender {
    pub object_type: ObjectType,
    pub position: Position,
    pub sprite_id: Option<String>,
}

impl From<&MapObject> for ObjectRender {
    fn from(obj: &MapObject) -> Self {
        Self {
            object_type: obj.object_type,
            position: obj.position,
            sprite_id: None, // Can be enhanced later with actual sprite IDs
        }
    }
}

/// Simplified render version of tile map for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileMapRender {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Vec<TileType>>,
}

impl From<&TileMap> for TileMapRender {
    fn from(map: &TileMap) -> Self {
        let tiles = map
            .tiles
            .iter()
            .map(|row| row.iter().map(|tile| tile.tile_type).collect())
            .collect();

        Self {
            width: map.width,
            height: map.height,
            tiles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tile_map() {
        let map = TileMap::new(10, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.tiles.len(), 10);
        assert_eq!(map.tiles[0].len(), 10);
    }

    #[test]
    fn test_add_and_get_object() {
        let mut map = TileMap::new(10, 10);
        let pos = Position::new(5.0, 5.0);
        let obj = MapObject::new(ObjectType::Terminal, pos);

        map.add_object(obj);

        let found = map.get_object_at(pos);
        assert!(found.is_some());
        assert_eq!(found.unwrap().object_type, ObjectType::Terminal);
    }

    #[test]
    fn test_remove_object() {
        let mut map = TileMap::new(10, 10);
        let pos = Position::new(5.0, 5.0);
        let obj = MapObject::new(ObjectType::Door, pos);

        map.add_object(obj);
        assert!(map.get_object_at(pos).is_some());

        let removed = map.remove_object_at(pos);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().object_type, ObjectType::Door);
        assert!(map.get_object_at(pos).is_none());
    }
}
