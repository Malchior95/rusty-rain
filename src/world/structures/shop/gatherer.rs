use std::collections::LinkedList;

use crate::{
    math::Pos,
    world::{
        World,
        actions::{ActionResult, gather_resource_action::GatherResourcesAction, store_action::StoreAction},
        inventory::Inventory,
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::Worker,
        world_map::{TileType, WorldMap, resources::ResourceType},
    },
};

pub struct Gatherer {
    pub inventory: Inventory,
    pub workers: Vec<GathererWorker>,
    pub resource_type: ResourceType,
}

pub struct GathererWorker {
    pub worker: Worker,
    pub action: GathererWorkerAction,
}

#[derive(Default)]
pub enum GathererWorkerAction {
    #[default]
    Idle,
    Store(StoreAction),
    GatherResouce(GatherResourcesAction),
}

impl Gatherer {
    pub fn build(
        world: &mut World,
        pos: Pos,
        resource_type: ResourceType,
    ) -> bool {
        if !world.map.can_build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT) {
            return false;
        }

        let gatherer = Self {
            inventory: Inventory::limited(10.0),
            workers: Vec::new(),
            resource_type,
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
            shop_type: ShopType::Gatherer(gatherer),
        };

        world.shops.push_back(shop);

        world
            .map
            .build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT, || TileType::Structure(ShopTypeDiscriminants::Gatherer));
        return true;
    }

    pub fn assign_worker(
        &mut self,
        worker: Worker,
    ) {
        //FIXME: check if can assign
        self.workers.push(GathererWorker {
            worker,
            action: GathererWorkerAction::default(),
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
            return; //gatherer cannot operate if no workers
        }

        //TODO: for now just find the main store. In the future - maybe find the closest one

        for worker in &mut self.workers {
            let maybe_new_action = match &mut worker.action {
                GathererWorkerAction::Idle => worker_start_work(&mut self.inventory, shops, map, &self.resource_type, structure.enterance),
                GathererWorkerAction::Store(store_action) => worker_continue_storing(store_action, shops, delta),
                GathererWorkerAction::GatherResouce(gather_resource_action) => {
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
) -> Option<GathererWorkerAction> {
    let result = gather_resource_action.process(map, inventory, delta);

    if let ActionResult::Completed = result {
        return Some(GathererWorkerAction::Idle);
    }
    None
}

fn worker_continue_storing(
    haul_data: &mut StoreAction,

    shops: &mut LinkedList<Shop>,
    delta: f32,
) -> Option<GathererWorkerAction> {
    //TODO: if no store - it means the store was destroyed! What do? For now - remain in the
    //current action, but do not progress
    //FIXME: how to ensure I find the same store that was ogirinally selected? Maybe check what
    //building is at path's end? better - introduce Ids and store id.
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

    Some(GathererWorkerAction::Idle)
}

fn worker_start_work(
    inventory: &mut Inventory,
    shops: &mut LinkedList<Shop>,
    map: &mut WorldMap,
    resource_type: &ResourceType,
    start: Pos,
) -> Option<GathererWorkerAction> {
    if inventory.is_full() {
        let position = shops.iter_mut().find_map(|s| {
            if let ShopType::MainStore(_) = &mut s.shop_type {
                return Some(s.structure.enterance);
            }
            return None;
        })?; // if no store - remain idle

        //TODO: for now haul everything - in the future: only haul some part at a time

        let haul_action = StoreAction::new(start, position, map, inventory)?;

        return Some(GathererWorkerAction::Store(haul_action));
    }

    //gather tree that is not being cut
    let gather_action = GatherResourcesAction::new(start, map, 10.0, |t| {
        if let TileType::Resource(rt, _, being_cut) = t {
            rt == resource_type && !being_cut
        } else {
            false
        }
    })?;

    Some(GathererWorkerAction::GatherResouce(gather_action))
}
