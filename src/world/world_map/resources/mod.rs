use log::info;
use strum_macros::Display;

use crate::world::inventory::{Inventory, InventoryItem};

#[derive(Display)]
pub enum ResourceType {
    Tree,
    Berries,
    Herbs,
    //TODO: more - clay, stone,...
}

impl PartialEq for ResourceType {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub struct ResourceCharge {
    pub item_type: InventoryItem,
    pub total: f32,
    pub current: f32,
}

impl ResourceCharge {
    pub fn gather(&mut self) -> Inventory {
        self.current -= 1.0;
        info!("Resource is being depleted. Type: {}, amount: {}", self.item_type, self.current);

        Inventory::from_iter([(self.item_type, 1.0)])
    }
}
