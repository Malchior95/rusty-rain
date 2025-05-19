use std::{collections::LinkedList, default};

use strum::IntoDiscriminant;

use crate::{
    ai,
    math::Pos,
    world::{
        World,
        actions::{GatherResourceAction, HaulAction, HaulActionResult},
        inventory::{Inventory, InventoryItem},
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::Worker,
        world_map::{TileType, WorldMap},
    },
};

use super::store::Store;

pub struct Woodcutter {
    pub inventory: Inventory,
    pub workers: Vec<WoodcutterWorker>,
}

pub struct WoodcutterWorker {
    pub worker: Worker,
    pub action: WoodcutterWorkerAction,
}

#[derive(Default)]
pub enum WoodcutterWorkerAction {
    #[default]
    Idle,
    Haul(HaulAction),
    GatherResouce(GatherResourceAction),
}

impl Woodcutter {
    pub fn build(world: &mut World, pos: Pos) -> bool {
        if !world.map.can_build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT) {
            return false;
        }

        let woodcutter = Self {
            inventory: Inventory::default(),
            workers: Vec::new(),
        };

        //FIXME: check if enterance is accessible...

        let structure = Structure {
            pos,
            height: Self::HEIGHT,
            width: Self::WIDTH,
            enterance: Pos::new(pos.x, pos.y - 1),
        };

        let shop = Shop {
            structure,
            shop_type: ShopType::Woodcutter(woodcutter),
        };

        world.shops.push_back(shop);

        world
            .map
            .build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT, || {
                TileType::Structure(ShopTypeDiscriminants::Woodcutter)
            });
        return true;
    }

    pub fn assign_worker(&mut self, worker: Worker) {
        //TODO: check if can assign
        self.workers.push(WoodcutterWorker {
            worker,
            action: WoodcutterWorkerAction::default(),
        });
    }

    pub fn process(
        &mut self,
        structure: &Structure,
        map: &mut WorldMap,
        shops: &mut LinkedList<Shop>,
        delta: f32,
    ) {
        if self.workers.is_empty() {
            return; //woodcutter cannot operate if no workers
        }

        //TODO: for now just find the main store. In the future - maybe find the closest one
        let mut closest_store = shops.iter_mut().find_map(|s| {
            if let ShopType::MainStore(ref mut store) = s.shop_type {
                return Some((store, s.structure.enterance));
            }
            return None;
        });

        for worker in &mut self.workers {
            let maybe_new_action = match worker.action {
                WoodcutterWorkerAction::Idle => worker_start_work(
                    &mut self.inventory,
                    &mut closest_store,
                    map,
                    structure.enterance,
                ),
                WoodcutterWorkerAction::Haul(ref mut haul_action) => {
                    worker_continue_hauling(haul_action, delta)
                }
                WoodcutterWorkerAction::GatherResouce(ref mut gather_resource_action) => {
                    worker_continue_gathering_resources(gather_resource_action, delta)
                }
            };

            if let Some(action) = maybe_new_action {
                worker.action = action;
            }
        }
    }

    pub const WIDTH: u8 = 2;
    pub const HEIGHT: u8 = 2;
}

fn worker_continue_gathering_resources(
    gather_resource_action: &mut GatherResourceAction,
    delta: f32,
) -> Option<WoodcutterWorkerAction> {
    None
}

fn worker_continue_hauling(
    haul_action: &mut HaulAction,
    delta: f32,
) -> Option<WoodcutterWorkerAction> {
    let result = haul_action.process(delta);
    match result {
        crate::world::actions::ActionResult::InProgress => {}
        crate::world::actions::ActionResult::Completed => {}
    };

    None
}

fn worker_start_work(
    inventory: &mut Inventory,
    maybe_store_and_position: &mut Option<(&mut Store, Pos)>,
    map: &mut WorldMap,
    start: Pos,
) -> Option<WoodcutterWorkerAction> {
    if inventory.is_full() {
        //new haul action
        if let Some(store_and_position) = maybe_store_and_position {
            for (key, value) in inventory.output.drain() {
                let stored_resource_value =
                    store_and_position.0.inventory.remove(&key).unwrap_or(0.0);
                //FIXME: actually, insert to Haul action - which will insert into store on
                //completion
                store_and_position
                    .0
                    .inventory
                    .insert(key, stored_resource_value + value);
            }
        }
        return None; //nowhere to store the wood - remain idle
    }

    //TODO: improve woodcutting - maybe each cutter has to cut a separate tree?

    let closest_tree_location =
        ai::pathfinding::breadth_first_closest(map, start, &TileType::Tree, true, true);

    if let Some(path) = closest_tree_location {
        //new cut tree action
        return None;
    }
    None //remain idle
}
