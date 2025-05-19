use std::collections::HashMap;

pub struct Inventory {
    pub input: HashMap<InventoryItem, f32>,
    pub output: HashMap<InventoryItem, f32>,
    pub output_limit: f32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
            output_limit: 10.0,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone)]
pub enum InventoryItem {
    Wood,
    Resin,
    Berries,
    Herbs,
    //TODO: more
}

impl Inventory {
    pub fn is_full(&self) -> bool {
        self.output.iter().map(|x| *x.1).sum::<f32>() > self.output_limit
    }
}
