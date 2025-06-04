use std::collections::LinkedList;

use log::info;

use crate::{
    config::inventory::InventoryItems,
    world::{
        World,
        actions::{ActionResult, BasicAction},
        inventory::Inventory,
        structures::BuildingBase,
        workers::{Worker, worker::WorkerActionResult},
    },
};

use super::shared;

pub struct HearthBehaviour {
    pub action: HearthAction,
    pub input: Inventory, //regular output can be taken from. inventory is private and treated
    //as input for processing
    pub unassigned_workers: LinkedList<Worker>,
}

pub enum HearthAction {
    Idle,
    Burning(BasicAction),
}

impl HearthBehaviour {
    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 4;

    pub const MAX_WORKERS: u8 = 1;

    pub const MATERIAL_SUPPLYING_THRESHOLD: f32 = 10.0;

    pub const WOOD_BURNING_RATE: f32 = 20.0;
}

impl Default for HearthBehaviour {
    fn default() -> Self {
        Self {
            action: HearthAction::Idle,
            input: Inventory::new(),
            unassigned_workers: LinkedList::new(),
        }
    }
}

impl HearthBehaviour {
    pub fn process(
        &mut self,
        shop_base: &mut BuildingBase,
        world: &mut World,
        delta: f32,
    ) {
        for _ in 0..shop_base.workers.len() {
            let mut worker = shop_base.workers.pop_front().unwrap();
            worker = self.process_worker(worker, world, shop_base, delta);
            shop_base.workers.push_back(worker);
        }

        let maybe_new_action = match &mut self.action {
            HearthAction::Burning(burning) => continue_burning(burning, delta),
            HearthAction::Idle => process_idle(&mut self.input, !shop_base.workers.is_empty()),
        };

        for _ in 0..self.unassigned_workers.len() {
            let mut worker = self.unassigned_workers.pop_front().unwrap();
            worker = worker.process_unassigned_worker(shop_base.pos, world, delta);

            self.unassigned_workers.push_back(worker);
        }

        if let Some(new_action) = maybe_new_action {
            self.action = new_action;
        }
    }

    pub fn process_worker(
        &mut self,
        worker: Worker,
        world: &mut World,
        shop_base: &mut BuildingBase,
        delta: f32,
    ) -> Worker {
        let shop_id = &"Hearth".to_string();
        let (mut worker, result) = worker.continue_action(shop_base.pos, delta, world, true);

        match result {
            WorkerActionResult::InProgress => {
                //continue action
            }

            WorkerActionResult::ProductionComplete(_) => {
                unreachable!("Hearth will never produce.");
            }

            WorkerActionResult::BroughtToShop(inventory) => {
                self.input.add_range(inventory);
            }

            WorkerActionResult::Idle => {
                //TODO: use any fuel
                //TODO: to function so that I can return early nicely
                if self.input.get(&InventoryItems::Wood) > HearthBehaviour::MATERIAL_SUPPLYING_THRESHOLD {
                    //no need to fetch fuel - stock full
                } else {
                    worker = shared::supply_command(worker, shop_base.pos, world, &vec![InventoryItems::Wood], shop_id);
                }
            }
        }
        worker
    }
}

fn continue_burning(
    action: &mut BasicAction,
    delta: f32,
) -> Option<HearthAction> {
    let result = action.continue_action(delta);

    if let ActionResult::Completed = result {
        return Some(HearthAction::Idle);
    }

    None
}

fn process_idle(
    inventory: &mut Inventory,
    has_worker: bool,
    //delta: f32,
) -> Option<HearthAction> {
    let wood = inventory.get(&InventoryItems::Wood);

    if wood > 1.0 && has_worker {
        inventory.remove(&InventoryItems::Wood, 1.0);

        let burning_action = BasicAction::new(HearthBehaviour::WOOD_BURNING_RATE);
        info!("Hearth has started burning, remaining fuel: {}", wood - 1.0);

        return Some(HearthAction::Burning(burning_action));
    };
    None

    //TODO: bad things should happen if the hearth is not burning
    //for now nothing happens
}
