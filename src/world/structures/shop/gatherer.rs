use log::info;

use crate::{
    ai::pathfinding,
    config::inventory::InventoryItems,
    world::{
        World,
        structures::BuildingBase,
        workers::{Worker, worker::WorkerActionResult},
        world_map::TileType,
    },
};

use super::shared;

pub struct GathererBehaviour {
    pub storing_all: bool,
}

impl GathererBehaviour {
    pub const WIDTH: u8 = 2;
    pub const HEIGHT: u8 = 2;

    pub const MAX_WORKERS: u8 = 2;
}

impl Default for GathererBehaviour {
    fn default() -> Self {
        Self {
            storing_all: Default::default(),
        }
    }
}

impl GathererBehaviour {
    pub fn process(
        &mut self,
        shop_base: &mut BuildingBase,
        world: &mut World,
        delta: f32,
    ) {
        //TODO: remoe this
        let shop_id = &format!("{}", shop_base.building);
        for _ in 0..shop_base.workers.len() {
            let worker = shop_base.workers.pop_front().unwrap();
            let (mut worker, result) = worker.continue_action(shop_base.pos, delta, world, false);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::ProductionComplete(_) => {
                    unreachable!("Gatherer will never produce.")
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    shop_base.output.add_range(inventory);
                }

                WorkerActionResult::Idle => {
                    if shop_base.output.is_full() || self.storing_all {
                        worker = shared::store_command(worker, world, &mut shop_base.output, shop_id);

                        //once started storing - store everything
                        if shop_base.output.total_items() <= 0.0 {
                            self.storing_all = false;
                        } else {
                            self.storing_all = true;
                        }
                    } else {
                        let resource_types = &shop_base.building.get_data().gathered_resource_types;
                        worker = gather_command(worker, world, resource_types, shop_id);
                    }
                }
            }
            shop_base.workers.push_back(worker);
        }
    }
}

fn gather_command(
    worker: Worker,
    world: &mut World,
    resource_items: &Vec<InventoryItems>,
    shop_id: &String,
) -> Worker {
    let idle_worker = if let Worker::Idle(idle_worker) = worker {
        idle_worker
    } else {
        return worker;
    };

    let maybe_path = pathfinding::dijkstra_closest(&world.map, idle_worker.pos, |t| {
        if let TileType::Resource(_, items, being_cut) = t {
            let main_item = &items.per_gather.first().unwrap().0;
            if resource_items.contains(main_item) && !being_cut {
                true
            } else {
                false
            }
        } else {
            false
        }
    });

    let path = if let Some(path) = maybe_path {
        path
    } else {
        info!("{} has no suitable resource nodes nearby.", shop_id);
        return Worker::Idle(idle_worker); //remain idle
    };

    return idle_worker.to_gathering(path, &mut world.map);
}
