use crate::world::{
    World,
    structures::{
        Shop, ShopType,
        shop::{gatherer::Gatherer, hearth::Hearth, store::Store},
    },
    world_map::resources::ResourceType,
};

impl World {
    pub fn get_gatherers(
        &self,
        resource_type: &ResourceType,
    ) -> Vec<&Shop<Gatherer>> {
        self.shops
            .iter()
            .filter_map(|s| {
                if let ShopType::Gatherer(g) = s {
                    if &g.data.resource_type == resource_type {
                        Some(g)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_hearths(&self) -> impl Iterator<Item = &Shop<Hearth>> {
        self.shops
            .iter()
            .filter_map(|s| if let ShopType::MainHearth(h) = s { Some(h) } else { None })
    }

    pub fn get_stores(&self) -> impl Iterator<Item = &Shop<Store>> {
        self.shops
            .iter()
            .filter_map(|s| if let ShopType::MainStore(h) = s { Some(h) } else { None })
    }
}
