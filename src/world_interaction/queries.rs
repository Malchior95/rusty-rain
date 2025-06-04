use crate::{
    config::inventory::InventoryItems,
    world::{
        World,
        building::{
            BuildingBase, BuildingBehaviour, StoreBehaviour,
            building_behaviour::{gatherer::GathererBehaviour, hearth::HearthBehaviour},
        },
    },
};

impl World {
    pub fn get_gatherers(
        &self,
        resource_type: &InventoryItems,
    ) -> impl Iterator<Item = (&BuildingBase, &GathererBehaviour)> {
        self.shops.iter().filter_map(move |s| {
            if let BuildingBehaviour::Gatherer(g) = &s.building_behaviour {
                if s.building_base
                    .building
                    .get_data()
                    .gathered_resource_types
                    .contains(resource_type)
                {
                    Some((&s.building_base, g))
                } else {
                    None
                }
            } else {
                None
            }
        })
    }

    pub fn get_hearths(&self) -> impl Iterator<Item = (&BuildingBase, &HearthBehaviour)> {
        self.shops.iter().filter_map(|s| {
            if let BuildingBehaviour::Hearth(h) = &s.building_behaviour {
                Some((&s.building_base, h))
            } else {
                None
            }
        })
    }

    pub fn get_stores(&self) -> impl Iterator<Item = (&BuildingBase, &StoreBehaviour)> {
        self.shops.iter().filter_map(|s| {
            if let BuildingBehaviour::Store(h) = &s.building_behaviour {
                Some((&s.building_base, h))
            } else {
                None
            }
        })
    }
}
