use crate::world::{
    World,
    structures::{Shop, ShopTypeDiscriminants},
    workers::worker::WorkerActionResult,
    world_map::resources::ResourceType,
};

use super::shared;

pub struct Gatherer {
    pub resource_type: ResourceType,
    pub storing_all: bool,
}

impl Gatherer {
    pub const WIDTH: u8 = 2;
    pub const HEIGHT: u8 = 2;

    pub const MAX_WORKERS: u8 = 2;
}

impl Shop<Gatherer> {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        let shop_id = &format!("Gatherer<{}>", self.data.resource_type);
        for _ in 0..self.workers.len() {
            let worker = self.workers.pop_front().unwrap();
            let (mut worker, result) =
                worker.continue_action(self.structure.pos, ShopTypeDiscriminants::Gatherer, delta, world);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::ProductionComplete(_) => {
                    unreachable!("Gatherer will never produce.")
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    shared::handle_supply_complete(inventory, &mut self.output, shop_id);
                }

                WorkerActionResult::Idle => {
                    if self.output.is_full() || self.data.storing_all {
                        worker =
                            shared::store_command(worker, world, &mut self.output, &mut self.data.storing_all, shop_id);
                    } else {
                        worker = shared::gather_command(worker, world, &self.data.resource_type, shop_id);
                    }
                }
            }
            self.workers.push_back(worker);
        }
    }
}
