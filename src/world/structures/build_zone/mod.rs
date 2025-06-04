use crate::world::{actions::BasicAction, inventory::Inventory};

use super::Building;

pub struct BuildZone {
    pub progress: BasicAction,
    pub materials_delivered: Inventory,
    pub building: Building,
}

impl BuildZone {
    pub fn new(shop_type: Building) -> Self {
        //FIXME: get build data
        let data = shop_type.building_base.building.get_data();
        Self {
            progress: BasicAction::new(data.build_time),
            materials_delivered: Inventory::new(),
            building: shop_type,
        }
    }

    pub fn is_delivery_complete(&self) -> bool {
        for (key, item) in &self.building.building_base.building.get_data().build_materials {
            if self.materials_delivered.get(key) < *item {
                return false;
            }
        }
        true
    }
}
