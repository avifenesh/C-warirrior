use super::validator::SuccessCriteria;
use crate::game::progression::LevelPrerequisites;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Function-Based Challenge System
// ============================================================================

/// Lesson content shown to player before the challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub title: String,
    pub content: Vec<String>,
    #[serde(default)]
    pub examples: Vec<LessonExample>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LessonExample {
    pub code: String,
    pub explanation: String,
}

/// Function signature that player must implement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<FunctionParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
}

/// Test case for function-based challenges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: Vec<serde_json::Value>,
    pub expected: String,
    #[serde(default)]
    pub sample: bool,
}

// ============================================================================
// Multi-Quest System
// ============================================================================

fn default_quest_xp() -> u32 {
    25
}

/// Progressive teaching content for each quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestTeaching {
    pub concept: String,
    pub explanation: String,
    #[serde(default)]
    pub tip: Option<String>,
}

/// A quest is a single challenge within a level
/// Each level can have multiple quests that must all be completed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quest {
    pub id: String,
    #[serde(default)]
    pub order: u32,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub recommended: bool,
    pub function_signature: FunctionSignature,
    pub user_template: String,
    pub test_cases: Vec<TestCase>,
    #[serde(default)]
    pub hints: Vec<String>,
    #[serde(default = "default_quest_xp")]
    pub xp_reward: u32,
    /// Progressive teaching content for this quest
    #[serde(default)]
    pub teaching: Option<QuestTeaching>,
}

/// Quest info for frontend display (includes completion status)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestInfo {
    pub id: String,
    pub order: u32,
    pub title: String,
    pub description: String,
    pub recommended: bool,
    pub completed: bool,
    pub xp_reward: u32,
    /// Progressive teaching content for this quest
    #[serde(default)]
    pub teaching: Option<QuestTeaching>,
}

// ============================================================================
// World Configuration
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WorldPreset {
    Tutorial,
    Corridor,
    Maze,
    Custom(String),
}

/// Terminal placement with optional quest link
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub x: f32,
    pub y: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quest_id: Option<String>,
}

/// Individual tile configuration for custom layouts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileConfig {
    pub x: usize,
    pub y: usize,
    #[serde(rename = "type")]
    pub tile_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldConfig {
    pub width: usize,
    pub height: usize,
    pub spawn_x: f32,
    pub spawn_y: f32,
    /// Legacy single terminal position (for backwards compatibility)
    #[serde(default)]
    pub terminal_x: f32,
    #[serde(default)]
    pub terminal_y: f32,
    /// Multiple terminals with quest links (preferred)
    #[serde(default)]
    pub terminals: Vec<TerminalConfig>,
    /// Custom tile placements (water, walls, doors, etc.)
    #[serde(default)]
    pub tiles: Option<Vec<TileConfig>>,
    /// Preset - now optional, ignored when tiles are provided
    #[serde(default)]
    pub preset: Option<WorldPreset>,
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
            terminals: vec![],
            tiles: None,
            preset: Some(WorldPreset::Tutorial),
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

    // Visual theme for level-specific tile rendering
    #[serde(default)]
    pub theme: Option<String>,

    // Multi-quest system (new)
    #[serde(default)]
    pub quests: Vec<Quest>,
    #[serde(default)]
    pub total_xp_reward: Option<u32>,

    // Function-based challenge system (legacy single-quest)
    #[serde(default)]
    pub lesson: Option<Lesson>,
    #[serde(default)]
    pub function_signature: Option<FunctionSignature>,
    #[serde(default)]
    pub user_template: Option<String>,
    #[serde(default)]
    pub test_cases: Vec<TestCase>,

    // Legacy fields (kept for backward compatibility during migration)
    #[serde(default)]
    pub code_template: String,
    #[serde(default)]
    pub success_criteria: Option<SuccessCriteria>,
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

    /// Check if this level uses the multi-quest system
    pub fn has_quests(&self) -> bool {
        !self.quests.is_empty()
    }

    /// Check if this level uses the new function-based challenge system
    pub fn is_function_based(&self) -> bool {
        self.function_signature.is_some() && !self.test_cases.is_empty()
    }

    /// Get quests for this level (backward compatible)
    /// Returns the quests array if defined, otherwise creates a single quest from legacy fields
    pub fn get_quests(&self) -> Vec<Quest> {
        if !self.quests.is_empty() {
            self.quests.clone()
        } else if let Some(ref sig) = self.function_signature {
            // Create single quest from legacy function-based fields
            vec![Quest {
                id: format!("{}_Q1", self.id),
                order: 1,
                title: self.title.clone(),
                description: self.description.clone(),
                recommended: true,
                function_signature: sig.clone(),
                user_template: self.user_template.clone().unwrap_or_default(),
                test_cases: self.test_cases.clone(),
                hints: self.hints.clone(),
                xp_reward: self.xp_reward,
                teaching: None,
            }]
        } else {
            // Legacy output-based level - no quests
            vec![]
        }
    }

    /// Get total quest count for this level
    pub fn quest_count(&self) -> usize {
        let quests = self.get_quests();
        if quests.is_empty() { 1 } else { quests.len() }
    }

    /// Get total XP reward for completing all quests
    pub fn get_total_xp(&self) -> u32 {
        if let Some(total) = self.total_xp_reward {
            total
        } else if !self.quests.is_empty() {
            self.quests.iter().map(|q| q.xp_reward).sum()
        } else {
            self.xp_reward
        }
    }

    /// Get a specific quest by ID
    pub fn get_quest(&self, quest_id: &str) -> Option<Quest> {
        self.get_quests().into_iter().find(|q| q.id == quest_id)
    }

    /// Get the template code for the player to start with
    pub fn get_template(&self) -> &str {
        self.user_template.as_deref().unwrap_or(&self.code_template)
    }

    /// Validate execution output against level criteria (legacy, only with compiler feature)
    #[cfg(feature = "compiler")]
    pub fn validate_output(&self, output: &crate::compiler::ExecutionOutput) -> bool {
        if !output.compile_success() {
            return false;
        }
        match &self.success_criteria {
            Some(criteria) => criteria.validate(output),
            None => false, // Function-based levels use test_cases instead
        }
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
    pub xp_reward: u32,
    // Quest progress fields
    pub total_quests: usize,
    pub completed_quests: usize,
    pub completion_percentage: f32,
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
            .map(|l| {
                let total_quests = l.quest_count();
                LevelInfo {
                    id: l.id.clone(),
                    title: l.title.clone(),
                    concept: l.concept.clone(),
                    completed: false, // Placeholder, logic for this belongs in GameState
                    locked: false,    // Placeholder
                    xp_reward: l.get_total_xp(),
                    total_quests,
                    completed_quests: 0, // Placeholder
                    completion_percentage: 0.0,
                }
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
