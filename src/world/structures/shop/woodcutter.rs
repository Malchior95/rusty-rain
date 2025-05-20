use std::collections::LinkedList;

use log::info;

use crate::{
    math::Pos,
    world::{
        World,
        actions::{ActionResult, BasicAction, gather_resource_action::GatherResourcesAction, store_action::StoreAction},
        inventory::{Inventory, InventoryItem},
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::Worker,
        world_map::{TileType, WorldMap},
    },
};

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
    Haul(StoreAction),
    GatherResouce(GatherResourcesAction),
}

impl Woodcutter {
    pub fn build(
        world: &mut World,
        pos: Pos,
    ) -> bool {
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

        world.map.build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT, || {
            TileType::Structure(ShopTypeDiscriminants::Woodcutter)
        });
        return true;
    }

    pub fn assign_worker(
        &mut self,
        worker: Worker,
    ) {
        //FIXME: check if can assign
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

        for worker in &mut self.workers {
            let maybe_new_action = match &mut worker.action {
                WoodcutterWorkerAction::Idle => worker_start_work(&mut self.inventory, shops, map, structure.enterance),
                WoodcutterWorkerAction::Haul(store_action) => worker_continue_storing(store_action, shops, delta),
                WoodcutterWorkerAction::GatherResouce(gather_resource_action) => {
                    worker_continue_gathering_resources(gather_resource_action, map, &mut self.inventory, delta)
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
    gather_resource_action: &mut GatherResourcesAction,
    map: &mut WorldMap,
    inventory: &mut Inventory,
    delta: f32,
) -> Option<WoodcutterWorkerAction> {
    let result = gather_resource_action.process(map, &mut inventory.output, delta);

    if let ActionResult::Completed = result {
        return Some(WoodcutterWorkerAction::Idle);
    }
    None
}

fn worker_continue_storing(
    haul_data: &mut StoreAction,

    shops: &mut LinkedList<Shop>,
    delta: f32,
) -> Option<WoodcutterWorkerAction> {
    //TODO: if no store - it means the store was destroyed! What do? For now - remain in the
    //current action, but do not progress
    //FIXME: how to ensure I find the same store that was ogirinally selected? Maybe check what
    //building is at path's end?
    let store = shops.iter_mut().find_map(|s| {
        if let ShopType::MainStore(store) = &mut s.shop_type {
            return Some(store);
        }
        return None;
    })?; // if no store - remain idle

    let result = haul_data.process(store, delta);

    if let ActionResult::InProgress = result {
        return None; //continue hauling - do not change state
    }

    Some(WoodcutterWorkerAction::Idle)
}

fn worker_start_work(
    inventory: &mut Inventory,
    shops: &mut LinkedList<Shop>,
    map: &mut WorldMap,
    start: Pos,
) -> Option<WoodcutterWorkerAction> {
    if inventory.is_full() {
        let position = shops.iter_mut().find_map(|s| {
            if let ShopType::MainStore(_) = &mut s.shop_type {
                return Some(s.structure.enterance);
            }
            return None;
        })?; // if no store - remain idle

        //TODO: for now haul everything - in the future: only haul some part at a time

        let haul_action = StoreAction::new(start, position, map, &mut inventory.output)?;

        return Some(WoodcutterWorkerAction::Haul(haul_action));
    }

    //TODO: improve woodcutting - maybe each cutter has to cut a separate tree?

    let gather_action = GatherResourcesAction::new(start, map, 10.0, |t| if let TileType::Tree(_) = t { true } else { false })?;

    Some(WoodcutterWorkerAction::GatherResouce(gather_action))
}
