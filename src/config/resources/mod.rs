use super::inventory::InventoryItem;

pub struct ResourceNode {
    pub name: String,
    pub output: Vec<(InventoryItem, f32)>,
}
