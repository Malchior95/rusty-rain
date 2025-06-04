use super::inventory::InventoryItems;

pub struct ResourceNode {
    pub name: String,
    pub output: Vec<(InventoryItems, f32)>,
}
