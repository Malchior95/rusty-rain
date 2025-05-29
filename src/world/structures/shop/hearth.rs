use log::info;

use crate::world::{
    World,
    actions::{ActionResult, BasicAction},
    inventory::{Inventory, InventoryItem},
    structures::{Shop, ShopTypeDiscriminants},
    workers::WorkerActionResult,
};

use super::shared;

pub struct Hearth {
    pub action: HearthAction,
    pub inventory: Inventory, //regular output can be taken from. inventory is private and treated
                              //as input for processing
}

pub enum HearthAction {
    Idle,
    Burning(BasicAction),
}

impl Hearth {
    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 4;

    pub const MAX_WORKERS: u8 = 1;

    pub const MATERIAL_SUPPLYING_THRESHOLD: f32 = 10.0;

    //TODO: maybe take workers inventory capacity into accout?
    pub const MIN_MATERIALS_TO_CONSIDER_SUPPLYING: f32 = 5.0;

    pub const WOOD_BURNING_RATE: f32 = 20.0;
}

impl Shop<Hearth> {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        self.process_worker_fuel_haul(world, delta);

        let maybe_new_action = match &mut self.data.action {
            HearthAction::Burning(burning) => continue_burning(burning, delta),
            HearthAction::Idle => process_idle(&mut self.data.inventory, !self.workers.is_empty()),
        };

        if let Some(new_action) = maybe_new_action {
            self.data.action = new_action;
        }
    }

    pub fn process_worker_fuel_haul(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        let shop_id = &"Hearth".to_string();
        for _ in 0..self.workers.len() {
            let worker = self.workers.pop_front().unwrap();

            let (mut worker, result) =
                worker.continue_action(self.structure.pos, ShopTypeDiscriminants::MainHearth, delta, world);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::ProductionComplete(_) => {
                    unreachable!("Hearth will never produce.");
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    shared::handle_supply_complete(inventory, &mut self.data.inventory, shop_id);
                }

                WorkerActionResult::Idle => {
                    //TODO: to function so that I can return early nicely
                    if self.data.inventory.get(&InventoryItem::Wood) > Hearth::MATERIAL_SUPPLYING_THRESHOLD {
                        //no need to fetch fuel - stock full
                    } else {
                        worker = shared::supply_command(
                            worker,
                            self.structure.pos,
                            world,
                            Hearth::MIN_MATERIALS_TO_CONSIDER_SUPPLYING,
                            InventoryItem::Wood,
                            shop_id,
                        );
                    }
                }
            }
            self.workers.push_back(worker);
        }
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
    let wood = inventory.get(&InventoryItem::Wood);

    if wood > 1.0 && has_worker {
        inventory.remove(&InventoryItem::Wood, 1.0);

        let burning_action = BasicAction::new(Hearth::WOOD_BURNING_RATE);
        info!("Hearth has started burning, remaining fuel: {}", wood - 1.0);

        return Some(HearthAction::Burning(burning_action));
    };
    None

    //TODO: bad things should happen if the hearth is not burning
    //for now nothing happens
}
