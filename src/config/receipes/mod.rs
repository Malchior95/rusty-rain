use std::sync::LazyLock;

use super::inventory::InventoryItem;

pub struct Receipe {
    pub input: Vec<Vec<(InventoryItem, f32)>>,
    pub output: Vec<(InventoryItem, f32)>,
    pub time_requirement: f32,
}
