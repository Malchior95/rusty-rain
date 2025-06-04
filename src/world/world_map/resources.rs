use strum_macros::Display;

use crate::config::inventory::InventoryItems;

use super::TileType;

#[derive(Display, PartialEq, Eq)]
pub enum ResourceType {
    Tree,
    Berries,
    Herbs,
    //TODO: more - clay, stone,...
}

pub struct ResourceCharge {
    pub per_gather: Vec<(InventoryItems, f32)>,
    pub total: f32,
    pub current: f32,
}

impl ResourceCharge {
    pub fn gather(&mut self) -> Vec<(InventoryItems, f32)> {
        self.current -= 1.0;
        self.per_gather.clone()
    }
}

impl ResourceType {
    pub fn tile_tree() -> TileType {
        TileType::Resource(
            ResourceType::Tree,
            ResourceCharge {
                per_gather: vec![(InventoryItems::Wood, 1.0), (InventoryItems::Resin, 0.2)],
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
                per_gather: vec![(InventoryItems::Berries, 1.0), (InventoryItems::Herbs, 0.2)],
                total: 50.0,
                current: 50.0,
            },
            false,
        )
    }
}
