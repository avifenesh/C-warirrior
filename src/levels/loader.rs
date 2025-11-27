use super::validator::SuccessCriteria;
use crate::game::progression::LevelPrerequisites;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldPreset {
    Tutorial,
    Corridor,
    Maze,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: usize,
    pub height: usize,
    pub spawn_x: f32,
    pub spawn_y: f32,
    pub terminal_x: f32,
    pub terminal_y: f32,
    pub preset: WorldPreset,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            width: 20,
            height: 15,
            spawn_x: 2.0 * 32.0,
            spawn_y: 2.0 * 32.0,
            terminal_x: 10.0 * 32.0,
            terminal_y: 7.0 * 32.0,
            preset: WorldPreset::Tutorial,
        }
    }
}

fn default_xp() -> u32 {
    50
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: String,
    pub prompt: String,
    pub expected_output: String,
    pub starter_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelData {
    pub id: String,
    pub title: String,
    pub concept: String,
    #[serde(default)]
    pub description: String,
    pub code_template: String,
    pub success_criteria: SuccessCriteria,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default = "default_xp")]
    pub xp_reward: u32,
    #[serde(default)]
    pub world_config: WorldConfig,
    #[serde(default)]
    pub map_file: Option<String>,
    #[serde(default)]
    pub challenges: Vec<Challenge>,
    #[serde(default)]
    pub prerequisites: LevelPrerequisites,
}

impl LevelData {
    pub fn from_json(json: &serde_json::Value) -> Result<Self, String> {
        serde_json::from_value(json.clone()).map_err(|e| format!("Failed to parse level: {}", e))
    }

    pub fn validate_output(&self, output: &crate::compiler::ExecutionOutput) -> bool {
        if !output.compile_success() {
            return false;
        }
        self.success_criteria.validate(output)
    }
}

pub struct LevelRegistry {
    levels: HashMap<String, LevelData>,
    order: Vec<String>,
    prerequisites: HashMap<String, LevelPrerequisites>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LevelInfo {
    pub id: String,
    pub title: String,
    pub concept: String,
    pub completed: bool,
    pub locked: bool,
}

impl LevelRegistry {
    /// Load levels from src/assets/levels.json
    pub fn load_from_json() -> Self {
        let json_str = include_str!("../assets/levels.json");
        let levels_json: Vec<serde_json::Value> =
            serde_json::from_str(json_str).expect("Invalid levels.json");

        let mut levels = HashMap::new();
        let mut order = Vec::new();
        let mut prerequisites = HashMap::new();

        for level_json in levels_json {
            let level = LevelData::from_json(&level_json).expect("Failed to parse level");
            order.push(level.id.clone());
            prerequisites.insert(level.id.clone(), level.prerequisites.clone());
            levels.insert(level.id.clone(), level);
        }

        Self {
            levels,
            order,
            prerequisites,
        }
    }

    pub fn get_level(&self, id: &str) -> Option<&LevelData> {
        self.levels.get(id)
    }

    /// Get the prerequisites map for all levels
    pub fn get_prerequisites(&self) -> &HashMap<String, LevelPrerequisites> {
        &self.prerequisites
    }

    pub fn get_all_info(&self) -> Vec<LevelInfo> {
        self.order
            .iter()
            .filter_map(|id| self.levels.get(id))
            .map(|l| LevelInfo {
                id: l.id.clone(),
                title: l.title.clone(),
                concept: l.concept.clone(),
                completed: false, // Placeholder, logic for this belongs in GameState
                locked: false,    // Placeholder
            })
            .collect()
    }

    pub fn get_next_level(&self, current_id: &str) -> Option<String> {
        let current_idx = self.order.iter().position(|id| id == current_id)?;
        self.order.get(current_idx + 1).cloned()
    }

    /// Get all level IDs in order
    pub fn get_level_order(&self) -> &[String] {
        &self.order
    }
}
