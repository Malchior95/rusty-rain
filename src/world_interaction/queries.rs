use crate::world::{
    World,
    actions::BasicAction,
    structures::{
        Shop, ShopType,
        shop::{gatherer::Gatherer, hearth::Hearth, store::Store},
    },
    workers::Worker,
    world_map::resources::ResourceType,
};

impl Worker {
    pub fn break_progress(&self) -> Option<&BasicAction> {
        match self {
            Worker::Idle(w) => Some(&w.break_progress),
            Worker::Supplying(w) => Some(&w.break_progress),
            Worker::Storing(w) => Some(&w.break_progress),
            Worker::Gathering(w) => Some(&w.break_progress),
            Worker::Returning(w) => Some(&w.break_progress),
            Worker::TakingBreak(w) => Some(&w.break_progress),
            Worker::Producing(w) => Some(&w.break_progress),
            Worker::Lost(_) => None,
        }
    }
}

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

    pub fn get_hearths(&self) -> Vec<&Shop<Hearth>> {
        self.shops
            .iter()
            .filter_map(|s| if let ShopType::MainHearth(h) = s { Some(h) } else { None })
            .collect()
    }

    pub fn get_stores(&self) -> Vec<&Shop<Store>> {
        self.shops
            .iter()
            .filter_map(|s| if let ShopType::MainStore(h) = s { Some(h) } else { None })
            .collect()
    }
}
