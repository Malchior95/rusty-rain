use crate::world::{
    actions::BasicAction,
    inventory::{Inventory, InventoryItems},
};

use super::ShopType;

pub struct BuildZone {
    pub progress: BasicAction,
    pub materials_required: &'static Vec<InventoryItems>,
    pub materials_delivered: Inventory,
    pub shop_type: ShopType,
}

impl BuildZone {
    pub fn new(shop_type: ShopType) -> Self {
        let data = shop_type.get_build_data();
        Self {
            progress: BasicAction::new(data.build_time),
            //TODO: maybe clone and mutate ->affected by perks and modifiers
            materials_required: &data.materials_required,
            materials_delivered: Inventory::new(),
            shop_type,
        }
    }

    pub fn is_delivery_complete(&self) -> bool {
        for (key, item) in self.materials_required {
            if self.materials_delivered.get(key) < *item {
                return false;
            }
        }
        true
    }
}
