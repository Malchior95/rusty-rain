use strum_macros::Display;

use crate::world::inventory::{Inventory, InventoryItem};

use super::TileType;

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
    pub per_gather: Inventory,
    pub total: f32,
    pub current: f32,
}

impl ResourceCharge {
    pub fn gather(&mut self) -> Inventory {
        self.current -= 1.0;
        self.per_gather.clone()
    }
}

impl ResourceType {
    pub fn tile_tree() -> TileType {
        TileType::Resource(
            ResourceType::Tree,
            ResourceCharge {
                per_gather: Inventory::from_iter([(InventoryItem::Wood, 1.0), (InventoryItem::Resin, 0.2)]),
                total: 10.0,
                current: 10.0,
            },
            false,
        )
    }

    pub fn tile_berry() -> TileType {
        TileType::Resource(
            ResourceType::Berries,
            ResourceCharge {
                per_gather: Inventory::from_iter([(InventoryItem::Berries, 1.0), (InventoryItem::Herbs, 0.2)]),
                total: 50.0,
                current: 50.0,
            },
            false,
        )
    }
}
