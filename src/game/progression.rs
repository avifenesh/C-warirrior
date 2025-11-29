use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Tracks player's progression through levels with non-linear prerequisites
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProgressionState {
    /// Levels that have been completed
    pub completed_levels: HashSet<String>,
    /// Levels that are currently unlocked (available to play)
    pub unlocked_levels: HashSet<String>,
    /// Total XP earned across all levels
    pub total_xp: u32,
    /// XP earned per level (for replay detection)
    pub level_xp: HashMap<String, u32>,
    /// Completed quests per level: level_id -> set of quest_ids
    #[serde(default)]
    pub completed_quests: HashMap<String, HashSet<String>>,
    /// XP earned per quest (for replay detection)
    #[serde(default)]
    pub quest_xp: HashMap<String, u32>,
}

/// Defines what prerequisites a level requires
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LevelPrerequisites {
    /// Level IDs that must ALL be completed (AND logic)
    #[serde(default)]
    pub requires_all: Vec<String>,
    /// Level IDs where ANY one must be completed (OR logic)
    #[serde(default)]
    pub requires_any: Vec<String>,
    /// Minimum total XP required
    #[serde(default)]
    pub min_xp: u32,
}

impl LevelPrerequisites {
    pub fn none() -> Self {
        Self::default()
    }

    /// Check if prerequisites are met given completed levels and XP
    pub fn is_satisfied(&self, completed: &HashSet<String>, total_xp: u32) -> bool {
        // Check XP requirement
        if total_xp < self.min_xp {
            return false;
        }

        // Check requires_all (AND logic)
        if !self.requires_all.iter().all(|req| completed.contains(req)) {
            return false;
        }

        // Check requires_any (OR logic) - empty means no requirement
        if !self.requires_any.is_empty()
            && !self.requires_any.iter().any(|req| completed.contains(req))
        {
            return false;
        }

        true
    }
}

impl ProgressionState {
    pub fn new() -> Self {
        let mut state = Self::default();
        // L01 is always unlocked by default
        state.unlocked_levels.insert("L01".to_string());
        state
    }

    /// Mark a level as completed and award XP
    /// Returns the XP earned (0 if already completed)
    pub fn complete_level(&mut self, level_id: &str, xp_reward: u32) -> u32 {
        let first_time = self.completed_levels.insert(level_id.to_string());

        if first_time {
            self.total_xp += xp_reward;
            self.level_xp.insert(level_id.to_string(), xp_reward);
            xp_reward
        } else {
            // Already completed - no XP reward on replay
            0
        }
    }

    /// Check if a level is completed
    pub fn is_completed(&self, level_id: &str) -> bool {
        self.completed_levels.contains(level_id)
    }

    /// Check if a level is unlocked
    pub fn is_unlocked(&self, level_id: &str) -> bool {
        self.unlocked_levels.contains(level_id)
    }

    /// Update unlocked levels based on prerequisites
    /// Call this after completing a level to unlock new ones
    pub fn update_unlocks(&mut self, prerequisites: &HashMap<String, LevelPrerequisites>) {
        for (level_id, prereqs) in prerequisites {
            if !self.unlocked_levels.contains(level_id)
                && prereqs.is_satisfied(&self.completed_levels, self.total_xp)
            {
                self.unlocked_levels.insert(level_id.clone());
            }
        }
    }

    /// Get list of levels that can be played right now
    pub fn get_available_levels(&self) -> Vec<&String> {
        self.unlocked_levels.iter().collect()
    }

    /// Get list of completed levels
    pub fn get_completed_levels(&self) -> Vec<&String> {
        self.completed_levels.iter().collect()
    }

    // ========================================================================
    // Quest-based progression methods
    // ========================================================================

    /// Mark a quest as completed and award XP
    /// Returns the XP earned (0 if already completed)
    pub fn complete_quest(&mut self, level_id: &str, quest_id: &str, xp_reward: u32) -> u32 {
        let level_quests = self.completed_quests.entry(level_id.to_string()).or_default();
        let first_time = level_quests.insert(quest_id.to_string());

        if first_time {
            self.total_xp += xp_reward;
            self.quest_xp.insert(quest_id.to_string(), xp_reward);
            xp_reward
        } else {
            // Already completed - no XP reward on replay
            0
        }
    }

    /// Check if a specific quest is completed
    pub fn is_quest_completed(&self, level_id: &str, quest_id: &str) -> bool {
        self.completed_quests
            .get(level_id)
            .map(|quests| quests.contains(quest_id))
            .unwrap_or(false)
    }

    /// Get count of completed quests for a level
    pub fn get_completed_quest_count(&self, level_id: &str) -> usize {
        self.completed_quests
            .get(level_id)
            .map(|quests| quests.len())
            .unwrap_or(0)
    }

    /// Get quest progress as (completed, total)
    pub fn get_quest_progress(&self, level_id: &str, total_quests: usize) -> (usize, usize) {
        let completed = self.get_completed_quest_count(level_id);
        (completed, total_quests)
    }

    /// Check if all quests in a level are completed
    pub fn is_level_fully_completed(&self, level_id: &str, total_quests: usize) -> bool {
        self.get_completed_quest_count(level_id) >= total_quests
    }

    /// Get set of completed quest IDs for a level
    pub fn get_completed_quests(&self, level_id: &str) -> HashSet<String> {
        self.completed_quests
            .get(level_id)
            .cloned()
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_progression_has_l01_unlocked() {
        let state = ProgressionState::new();
        assert!(state.is_unlocked("L01"));
        assert!(!state.is_completed("L01"));
    }

    #[test]
    fn test_complete_level_awards_xp() {
        let mut state = ProgressionState::new();
        let xp = state.complete_level("L01", 50);
        assert_eq!(xp, 50);
        assert_eq!(state.total_xp, 50);
        assert!(state.is_completed("L01"));
    }

    #[test]
    fn test_replay_awards_no_xp() {
        let mut state = ProgressionState::new();
        state.complete_level("L01", 50);
        let xp = state.complete_level("L01", 50);
        assert_eq!(xp, 0);
        assert_eq!(state.total_xp, 50);
    }

    #[test]
    fn test_prerequisites_requires_all() {
        let prereqs = LevelPrerequisites {
            requires_all: vec!["L01".to_string(), "L02".to_string()],
            requires_any: vec![],
            min_xp: 0,
        };

        let mut completed = HashSet::new();
        assert!(!prereqs.is_satisfied(&completed, 0));

        completed.insert("L01".to_string());
        assert!(!prereqs.is_satisfied(&completed, 0));

        completed.insert("L02".to_string());
        assert!(prereqs.is_satisfied(&completed, 0));
    }

    #[test]
    fn test_prerequisites_requires_any() {
        let prereqs = LevelPrerequisites {
            requires_all: vec![],
            requires_any: vec!["L02".to_string(), "L03".to_string()],
            min_xp: 0,
        };

        let mut completed = HashSet::new();
        assert!(!prereqs.is_satisfied(&completed, 0));

        completed.insert("L02".to_string());
        assert!(prereqs.is_satisfied(&completed, 0));
    }

    #[test]
    fn test_prerequisites_min_xp() {
        let prereqs = LevelPrerequisites {
            requires_all: vec![],
            requires_any: vec![],
            min_xp: 100,
        };

        let completed = HashSet::new();
        assert!(!prereqs.is_satisfied(&completed, 50));
        assert!(prereqs.is_satisfied(&completed, 100));
        assert!(prereqs.is_satisfied(&completed, 150));
    }

    #[test]
    fn test_update_unlocks() {
        let mut state = ProgressionState::new();
        let mut prereqs = HashMap::new();

        // L02 requires L01
        prereqs.insert(
            "L02".to_string(),
            LevelPrerequisites {
                requires_all: vec!["L01".to_string()],
                requires_any: vec![],
                min_xp: 0,
            },
        );

        // L05 requires L02 AND L04
        prereqs.insert(
            "L05".to_string(),
            LevelPrerequisites {
                requires_all: vec!["L02".to_string(), "L04".to_string()],
                requires_any: vec![],
                min_xp: 0,
            },
        );

        // Before completing anything
        state.update_unlocks(&prereqs);
        assert!(!state.is_unlocked("L02"));

        // Complete L01
        state.complete_level("L01", 50);
        state.update_unlocks(&prereqs);
        assert!(state.is_unlocked("L02"));
        assert!(!state.is_unlocked("L05"));

        // Complete L02 and L04
        state.complete_level("L02", 75);
        state.unlocked_levels.insert("L04".to_string()); // Assume L04 was unlocked some other way
        state.complete_level("L04", 125);
        state.update_unlocks(&prereqs);
        assert!(state.is_unlocked("L05"));
    }
}
