use std::collections::HashMap;

use log::info;
use strum_macros::EnumDiscriminants;

use crate::{math::Pos, world::inventory::InventoryItem};

use super::{TileType, WorldMap};

#[derive(EnumDiscriminants)]
pub enum ResourceType {
    Berries(ResourceCharge),
    Herbs(ResourceCharge),
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
    pub fn gather(&mut self) -> HashMap<InventoryItem, f32> {
        self.current -= 1.0;
        info!("Resource is being depleted. Type: {}, amount: {}", self.item_type, self.current);

        HashMap::from_iter([(self.item_type, 1.0)])
    }
}
