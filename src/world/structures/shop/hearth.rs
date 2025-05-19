use std::collections::{HashMap, LinkedList};

use log::info;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{
        World,
        actions::{ActionResult, BasicAction, supply_action::SupplyAction},
        inventory::InventoryItem,
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::Worker,
        world_map::{TileType, WorldMap},
    },
};

pub struct Hearth {
    pub action: HearthAction,
    pub inventory: HashMap<InventoryItem, f32>,
    pub hearth_worker: Option<HearthWorker>,
}

pub struct HearthWorker {
    pub worker: Worker,
    pub action: HearthWorkerAction,
}

#[derive(Default)]
pub enum HearthWorkerAction {
    #[default]
    Idle,
    Haul(SupplyAction),
}

#[derive(Default)]
pub enum HearthAction {
    #[default]
    Idle,
    Burning(BasicAction),
}

impl Hearth {
    pub fn build(
        world: &mut World,
        pos: Pos,
    ) -> bool {
        if !world.map.can_build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT) {
            return false;
        }

        let hearth = Self {
            action: HearthAction::default(),
            inventory: HashMap::from_iter([(InventoryItem::Wood, 10.0)]),
            hearth_worker: None,
        };

        //FIXME: check if enterance is accessible

        let structure = Structure {
            pos,
            height: Hearth::HEIGHT,
            width: Hearth::WIDTH,
            enterance: Pos::new(pos.x, pos.y - 1),
        };

        let shop = Shop {
            structure,
            shop_type: ShopType::MainHearth(hearth),
        };

        world.shops.push_back(shop);

        world.map.build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT, || {
            TileType::Structure(ShopTypeDiscriminants::MainHearth)
        });
        return true;
    }

    pub fn assign_worker(
        &mut self,
        worker: Worker,
    ) {
        //FIXME:check if worker can be assigned

        self.hearth_worker = Some(HearthWorker {
            worker,
            action: HearthWorkerAction::Idle,
        });
    }

    pub fn process(
        &mut self,
        structure: &Structure,
        map: &mut WorldMap,
        shops: &mut LinkedList<Shop>,
        delta: f32,
    ) {
        if let Some(worker) = &mut self.hearth_worker {
            process_worker_fuel_haul(worker, &mut self.inventory, structure, map, shops, delta);
        }

        let maybe_new_action = match &mut self.action {
            HearthAction::Burning(burning) => process_burning(burning, delta),
            HearthAction::Idle => process_idle(&mut self.inventory, self.hearth_worker.is_some()),
        };

        if let Some(new_action) = maybe_new_action {
            self.action = new_action;
        }
    }

    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 4;
}

fn process_worker_fuel_haul(
    worker: &mut HearthWorker,
    inventory: &mut HashMap<InventoryItem, f32>,
    structure: &Structure,
    map: &WorldMap,
    shops: &mut LinkedList<Shop>,

    delta: f32,
) {
    if *inventory.get(&InventoryItem::Wood).unwrap_or(&0.0) > 10.0 {
        //no need to fetch fuel - stock full
        return;
    }

    let maybe_new_action = match &mut worker.action {
        HearthWorkerAction::Idle => worker_start_hauling(structure, map, shops),
        HearthWorkerAction::Haul(haul_data) => worker_continue_hauling(inventory, haul_data, delta),
    };

    if let Some(new_action) = maybe_new_action {
        worker.action = new_action;
    };
}

fn process_burning(
    action: &mut BasicAction,
    delta: f32,
) -> Option<HearthAction> {
    let result = action.process(delta);

    if let ActionResult::Completed = result {
        return Some(HearthAction::Idle);
    }

    None
}

fn process_idle(
    inventory: &mut HashMap<InventoryItem, f32>,
    has_worker: bool,
    //delta: f32,
) -> Option<HearthAction> {
    //this is weird... why do I need to dereference a float?
    let wood = *inventory.get(&InventoryItem::Wood).unwrap_or(&0.0);

    //if idle for the first time, this is expected - start new burning process if wood supply
    //allows and worker is present

    if wood > 1.0 && has_worker {
        inventory.insert(InventoryItem::Wood, wood - 1.0);

        let burning_action = BasicAction::new(10.0);
        info!("Hearth has started burning, remaining fuel: {}", wood - 1.0);

        return Some(HearthAction::Burning(burning_action));
    };
    None

    //TODO: bad things should happen if the hearth is not burning
    //for now nothing happens
}

fn worker_start_hauling(
    structure: &Structure,
    map: &WorldMap,
    shops: &mut LinkedList<Shop>,
    //delta: f32,
) -> Option<HearthWorkerAction> {
    info!("Worker wants to start supplying 10 wood!");
    //TODO: for now - return main store. In the future maybe search the closest store? Maybe
    //search woodcutters, if closer?
    let (store, position) = shops.iter_mut().find_map(|s| {
        if let ShopType::MainStore(ref mut store) = s.shop_type {
            return Some((store, s.structure.enterance));
        }
        None
    })?;

    let haul_action = SupplyAction::new(structure.enterance, position, map, HashMap::from_iter([(InventoryItem::Wood, 10.0)]), store)?;

    Some(HearthWorkerAction::Haul(haul_action))
}

fn worker_continue_hauling(
    inventory: &mut HashMap<InventoryItem, f32>,
    haul_data: &mut SupplyAction,
    delta: f32,
) -> Option<HearthWorkerAction> {
    let result = haul_data.process(inventory, delta);

    if let ActionResult::InProgress = result {
        return None; //continue hauling - do not change state
    }

    Some(HearthWorkerAction::Idle)
}
