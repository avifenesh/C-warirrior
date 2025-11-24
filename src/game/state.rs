use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::constants::TILE_SIZE;
use super::inventory::Inventory;
use super::map::{ObjectRender, TileMapRender};
use super::physics;
use super::player::{Direction, Player};
use super::world::{Tile, TileType, World};

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GamePhase {
    MainMenu,
    Playing,
    Coding, // Player is at terminal, writing code
    Paused,
    LevelComplete,
}

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

    /// Move player in a direction, returns true if movement was successful
    pub fn move_player(&mut self, direction: Direction, distance: f32) -> bool {
        // Can't move if not in playing state
        if self.game_phase != GamePhase::Playing {
            return false;
        }

        // Calculate new position
        let new_position = physics::calculate_movement(self.player.position, direction, distance);

        // Check for collision
        if physics::check_collision(&self.world, self.player.position, new_position) {
            return false; // Movement blocked
        }

        // Update player position and facing direction
        self.player.position = new_position;
        self.player.facing = direction;

        true
    }

    /// Check if a position is walkable
    pub fn is_position_walkable(&self, x: f32, y: f32) -> bool {
        physics::is_position_walkable(&self.world, Position::new(x, y))
    }

    /// Interact with the nearest interactable object (terminal, door, etc.)
    /// Returns Some(TileType) if an interactable was found and activated
    pub fn interact_with_nearest(&mut self) -> Option<TileType> {
        // Can only interact while playing
        if self.game_phase != GamePhase::Playing {
            return None;
        }

        // Find nearest interactable within 1.5 tiles
        let interaction_range = TILE_SIZE * 1.5;
        let nearest = physics::find_nearest_interactable(
            &self.world,
            self.player.position,
            interaction_range,
        );

        if let Some((_x, _y, tile_type)) = nearest {
            match tile_type {
                TileType::Terminal => {
                    self.enter_coding_mode();
                    return Some(TileType::Terminal);
                }
                TileType::Door => {
                    // Door interaction logic would go here
                    return Some(TileType::Door);
                }
                _ => {}
            }
        }

        None
    }

    /// Update game state for a single tick
    pub fn update(&mut self, delta: Duration) {
        // Update logic that runs every tick
        // This is where time-based game mechanics would go

        // For now, this is a placeholder for future game tick logic
        // Examples of what could go here:
        // - Animation updates
        // - Particle effects
        // - Enemy AI (future)
        // - Environmental effects
        // - Status effect timers

        let _delta_secs = delta.as_secs_f32();

        // Currently no tick-based logic needed
        // Game state is updated through player actions only
    }

    /// Generate render state for frontend (20x15 viewport centered on player)
    pub fn to_render_state(&self) -> RenderState {
        let viewport_width = 20;
        let viewport_height = 15;

        // Calculate viewport offset (center on player)
        let (px, py) = self.player.position.tile_coords(TILE_SIZE);
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
            map: None, // Can be populated when we have a TileMap in GameState
            objects: Vec::new(), // Can be populated when we have objects in GameState
            show_terminal: self.game_phase == GamePhase::Coding,
            active_dialogue: None, // Can be populated when we add dialogue system
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderState {
    pub player: Player,
    pub visible_tiles: Vec<Vec<Tile>>,
    pub viewport_offset: Position,
    pub game_phase: GamePhase,
    pub current_level_id: Option<String>,
    pub map: Option<TileMapRender>,
    pub objects: Vec<ObjectRender>,
    pub show_terminal: bool,
    pub active_dialogue: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlayerAction {
    Move { direction: Direction },
    Interact,
    SubmitCode { code: String },
    OpenInventory,
    UseItem { item_id: String },
    Pause,
    Resume,
}
