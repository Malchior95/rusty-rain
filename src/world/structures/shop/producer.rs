use log::info;

use crate::world::{
    World,
    inventory::{Inventory, InventoryItem, InventoryItems},
    receipes::Receipe,
    structures::{Shop, ShopTypeDiscriminants},
    workers::WorkerActionResult,
};

use super::shared;

pub struct Producer {
    pub storing_all: bool,
    pub receipe: Receipe,
    pub input: Inventory,
}

impl Shop<Producer> {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        let shop_id = &format!("Producer<{}>", self.data.receipe);
        for _ in 0..self.workers.len() {
            let worker = self.workers.pop_front().unwrap();

            let (mut worker, result) =
                worker.continue_action(self.structure.pos, ShopTypeDiscriminants::Producer, delta, world);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    shared::handle_supply_complete(inventory, &mut self.data.input, shop_id);
                }

                WorkerActionResult::ProductionComplete(receipe) => {
                    shared::handle_poruction_complete(&mut self.data.input, &mut self.output, receipe);
                }

                WorkerActionResult::Idle => {
                    //TODO: make this a function - too much logic, cannot nicely return early from
                    //match statement
                    if self.output.is_full() || self.data.storing_all {
                        worker =
                            shared::store_command(worker, world, &mut self.output, &mut self.data.storing_all, shop_id);
                    } else
                    //if receipe requirements met - produce, otherwise, supply
                    if let Some((materials_to_supply, amount)) =
                        get_missing_materials(&self.data.receipe.input, &self.data.input)
                    {
                        info!(
                            "{} is missing {} {} to start producing",
                            shop_id, materials_to_supply, amount
                        );

                        info!("{} only has {}", shop_id, &self.data.input);
                        worker = shared::supply_command(
                            worker,
                            self.structure.pos,
                            world,
                            amount,
                            materials_to_supply,
                            shop_id,
                        );
                    } else {
                        worker = shared::produce_command(worker, self.data.receipe.clone(), shop_id);
                    }
                }
            }
            self.workers.push_back(worker);
        }
    }
}

fn get_missing_materials(
    receipe_input: &Vec<InventoryItems>,
    current_store: &Inventory,
) -> Option<(InventoryItem, f32)> {
    for &(key, item) in receipe_input {
        if current_store.get(&key) < item {
            return Some((key, item));
        }
    }
    None
}
