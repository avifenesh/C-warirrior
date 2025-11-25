use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::game::progression::ProgressionState;
use crate::game::state::Position;

/// Serializable save data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub slot_name: String,
    pub progression: ProgressionState,
    pub current_level_id: Option<String>,
    pub player_position: Position,
    pub timestamp: u64,
}

impl SaveData {
    pub const CURRENT_VERSION: u32 = 1;

    pub fn new(slot_name: String) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            slot_name,
            progression: ProgressionState::new(),
            current_level_id: None,
            player_position: Position::new(0.0, 0.0),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }
}

/// Manages save/load operations
pub struct SaveManager {
    save_dir: PathBuf,
}

impl SaveManager {
    /// Create a new SaveManager with the default save directory
    pub fn new() -> Result<Self, String> {
        let save_dir = Self::get_save_directory()?;
        fs::create_dir_all(&save_dir)
            .map_err(|e| format!("Failed to create save directory: {}", e))?;
        Ok(Self { save_dir })
    }

    /// Get the platform-appropriate save directory
    fn get_save_directory() -> Result<PathBuf, String> {
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| "HOME environment variable not set".to_string())?;
            Ok(PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("code-warrior")
                .join("saves"))
        }

        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA")
                .map_err(|_| "APPDATA environment variable not set".to_string())?;
            Ok(PathBuf::from(appdata)
                .join("code-warrior")
                .join("saves"))
        }

        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME")
                .map_err(|_| "HOME environment variable not set".to_string())?;
            Ok(PathBuf::from(home)
                .join(".config")
                .join("code-warrior")
                .join("saves"))
        }
    }

    /// Get the file path for a save slot
    fn get_save_path(&self, slot_name: &str) -> PathBuf {
        self.save_dir.join(format!("{}.json", slot_name))
    }

    /// Save game data to a slot
    pub fn save(&self, data: &SaveData) -> Result<(), String> {
        let path = self.get_save_path(&data.slot_name);
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Failed to serialize save data: {}", e))?;
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write save file: {}", e))?;
        Ok(())
    }

    /// Load game data from a slot
    pub fn load(&self, slot_name: &str) -> Result<SaveData, String> {
        let path = self.get_save_path(slot_name);
        let json = fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read save file: {}", e))?;
        let data: SaveData = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse save data: {}", e))?;

        // Check version compatibility
        if data.version > SaveData::CURRENT_VERSION {
            return Err(format!(
                "Save file version {} is newer than supported version {}",
                data.version, SaveData::CURRENT_VERSION
            ));
        }

        Ok(data)
    }

    /// Delete a save slot
    pub fn delete(&self, slot_name: &str) -> Result<(), String> {
        let path = self.get_save_path(slot_name);
        if path.exists() {
            fs::remove_file(&path)
                .map_err(|e| format!("Failed to delete save file: {}", e))?;
        }
        Ok(())
    }

    /// List all available save slots
    pub fn list_saves(&self) -> Result<Vec<SaveSlotInfo>, String> {
        let mut saves = Vec::new();

        let entries = fs::read_dir(&self.save_dir)
            .map_err(|e| format!("Failed to read save directory: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(data) = serde_json::from_str::<SaveData>(&json) {
                        saves.push(SaveSlotInfo {
                            slot_name: data.slot_name.clone(),
                            timestamp: data.timestamp,
                            total_xp: data.progression.total_xp,
                            levels_completed: data.progression.completed_levels.len(),
                            current_level: data.current_level_id.clone(),
                        });
                    }
                }
            }
        }

        // Sort by timestamp (newest first)
        saves.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(saves)
    }

    /// Check if a save slot exists
    pub fn exists(&self, slot_name: &str) -> bool {
        self.get_save_path(slot_name).exists()
    }
}

impl Default for SaveManager {
    fn default() -> Self {
        Self::new().expect("Failed to create SaveManager")
    }
}

/// Summary info about a save slot (for UI)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotInfo {
    pub slot_name: String,
    pub timestamp: u64,
    pub total_xp: u32,
    pub levels_completed: usize,
    pub current_level: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_manager() -> SaveManager {
        let dir = tempdir().unwrap();
        SaveManager {
            save_dir: dir.keep(),
        }
    }

    #[test]
    fn test_save_and_load() {
        let manager = create_test_manager();
        let mut data = SaveData::new("test_slot".to_string());
        data.progression.complete_level("L01", 50);

        manager.save(&data).unwrap();
        let loaded = manager.load("test_slot").unwrap();

        assert_eq!(loaded.slot_name, "test_slot");
        assert_eq!(loaded.progression.total_xp, 50);
        assert!(loaded.progression.is_completed("L01"));
    }

    #[test]
    fn test_list_saves() {
        let manager = create_test_manager();

        // Create a few saves
        for i in 1..=3 {
            let data = SaveData::new(format!("slot_{}", i));
            manager.save(&data).unwrap();
        }

        let saves = manager.list_saves().unwrap();
        assert_eq!(saves.len(), 3);
    }

    #[test]
    fn test_delete_save() {
        let manager = create_test_manager();
        let data = SaveData::new("to_delete".to_string());
        manager.save(&data).unwrap();

        assert!(manager.exists("to_delete"));
        manager.delete("to_delete").unwrap();
        assert!(!manager.exists("to_delete"));
    }
}
