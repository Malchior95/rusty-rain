use std::{collections::LinkedList, error};

use log::{error, info};

use crate::{
    ai::pathfinding::{self, pathfinding_helpers},
    math::Pos,
    world::{
        World,
        actions::ActionResult,
        inventory::{Inventory, InventoryItem},
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::{Worker, WorkerActionResult},
        world_map::{TileType, WorldMap, resources::ResourceType},
    },
};

pub struct Gatherer {
    pub resource_type: ResourceType,
    pub storing_all: bool,
}

impl Gatherer {
    pub const WIDTH: u8 = 2;
    pub const HEIGHT: u8 = 2;

    pub const MAX_WORKERS: u8 = 2;
}

pub fn build_gatherer<'a>(
    world: &'a mut World,
    pos: Pos,
    resource_type: ResourceType,
) -> Option<&'a mut Shop<Gatherer>> {
    if !world.map.can_build(pos.x, pos.y, Gatherer::WIDTH, Gatherer::HEIGHT) {
        return None;
    }

    let gatherer = Gatherer {
        resource_type,
        storing_all: false,
    };

    let shop = Shop {
        structure: Structure {
            pos,
            height: Gatherer::HEIGHT,
            width: Gatherer::WIDTH,
        },
        workers: Vec::with_capacity(Gatherer::MAX_WORKERS as usize),
        max_workers: Gatherer::MAX_WORKERS,
        output: Inventory::limited(10.0),
        data: gatherer,
    };

    let shop_type = ShopType::Gatherer(shop);

    world.map.build(pos.x, pos.y, Gatherer::WIDTH, Gatherer::HEIGHT, || {
        TileType::Structure(ShopTypeDiscriminants::Gatherer)
    });

    world.shops.push_back(shop_type);

    //return to user for modifications
    if let ShopType::Gatherer(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    panic!("std lib failed");
}

impl Shop<Gatherer> {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        for worker in &mut self.workers {
            let mut result = worker.continue_action(delta, self.structure.pos, &mut world.map);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::BroughtToStore(ref mut inventory, pos) => {
                    if inventory.is_empty() {
                        error!("Worker was bringing empty inventory to the store!");
                        continue;
                    }

                    //TODO: in the future I might want to store not only in the store, but in the
                    //closest shop that requires those resources. Then I would not want to put to
                    //output (as in the store), but to the input or data.inventory (as in Hearth)
                    let maybe_store = world.shops.iter_mut().find(|s| s.location() == pos);

                    let store = if let Some(store) = maybe_store {
                        store
                    } else {
                        //store is not where it was before - return to building?

                        error!("Worker was bringing items to the store, but the store is gone!");
                        //FIXME:
                        panic!()
                    };

                    info!("The following materials were added to Store's inventory: {}", inventory);

                    store.inventory_mut().add_range(inventory.drain());
                    info!("The store now has: {}", store.inventory());
                }

                WorkerActionResult::BroughtToShop(ref mut inventory) => {
                    if inventory.is_empty() {
                        continue;
                    }
                    info!(
                        "The following materials were added to Gatherer <{}> inventory: {}",
                        self.data.resource_type, inventory
                    );
                    self.output.add_range(inventory.drain());

                    info!(
                        "Gatherer <{}> has total items: {}",
                        self.data.resource_type, self.output
                    );
                }

                WorkerActionResult::Idle => {
                    if let Worker::Idle(idle_worker) = worker {
                        if self.output.is_full() || self.data.storing_all {
                            //store items
                            info!(
                                "Gatherer <{}> is storing resources. Current inventory: {}",
                                self.data.resource_type, self.output
                            );

                            let (closest_shop, path) = if let Some(x) = pathfinding_helpers::closest_shop(
                                self.structure.pos,
                                &world.map,
                                &mut world.shops,
                                |s| {
                                    //TODO: just bring to the store. In the future - maybe consider
                                    //bringing to the closest shop that needs materials?
                                    if let ShopType::MainStore(_) = s { true } else { false }
                                },
                            ) {
                                x
                            } else {
                                info!("Gatherer <{}> has no suitable stores nearby.", self.data.resource_type);
                                continue; //remain idle
                            };

                            *worker = idle_worker.to_storing(&world.map, path, &mut self.output);

                            //once started storing - store everything
                            if self.output.total_items() <= 0.0 {
                                self.data.storing_all = false;
                            } else {
                                self.data.storing_all = true;
                            }

                            continue;
                        }

                        //else gather

                        let maybe_path = pathfinding::dijkstra_closest(&world.map, idle_worker.pos, |t| {
                            if let TileType::Resource(r, _, being_cut) = t {
                                if *r == self.data.resource_type && !being_cut {
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
                            info!(
                                "Gatherer <{}> has no suitable resource nodes nearby.",
                                self.data.resource_type
                            );
                            continue; //remain idle
                        };

                        *worker = idle_worker.to_gathering(path, &mut world.map);
                    }
                }
            }
        }
    }
}
