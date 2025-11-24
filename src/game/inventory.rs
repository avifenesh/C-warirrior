use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub description: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemType {
    Key,
    Weapon,
    Consumable,
    QuestItem,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub max_slots: usize,
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        Self {
            items: Vec::new(),
            max_slots,
        }
    }

    pub fn add_item(&mut self, item: Item) -> bool {
        if self.items.len() < self.max_slots {
            // Check if item already exists (stack)
            if let Some(existing) = self.items.iter_mut().find(|i| i.id == item.id) {
                existing.quantity += item.quantity;
            } else {
                self.items.push(item);
            }
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, item_id: &str) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|i| i.id == item_id) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }
}
