use crate::world::{World, structures::Shop, workers::WorkerActionResult, world_map::resources::ResourceType};

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
        for worker in &mut self.workers {
            let mut result = worker.continue_action(delta, self.structure.pos, &mut world.map);

            match &mut result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::ProductionComplete(_) => {
                    unreachable!("Gatherer will never produce.")
                }

                WorkerActionResult::BroughtToStore(inventory, pos) => {
                    shared::handle_storing_complete(inventory, *pos, world, shop_id);
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    shared::handle_supply_complete(inventory, &mut self.output, shop_id);
                }

                WorkerActionResult::Idle => {
                    if self.output.is_full() || self.data.storing_all {
                        shared::store_command(
                            worker,
                            world,
                            self.structure.pos,
                            &mut self.output,
                            &mut self.data.storing_all,
                            shop_id,
                        );
                        continue;
                    }

                    shared::gather_command(worker, world, &self.data.resource_type, shop_id);
                }
            }
        }
    }
}
