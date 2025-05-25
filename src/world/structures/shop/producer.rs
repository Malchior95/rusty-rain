use log::info;

use crate::world::{
    World,
    inventory::{Inventory, InventoryItem},
    receipes::Receipe,
    structures::Shop,
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
        for worker in &mut self.workers {
            let mut result = worker.continue_action(delta, self.structure.pos, world);

            match &mut result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::BroughtToStore(inventory, pos) => {
                    shared::handle_storing_complete(inventory, *pos, world, shop_id);
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    shared::handle_supply_complete(inventory, &mut self.data.input, shop_id);
                }

                WorkerActionResult::ProductionComplete(receipe) => {
                    shared::handle_poruction_complete(&mut self.data.input, &mut self.output, receipe);
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

                    //if receipe requirements met - produce, otherwise, supply

                    let maybe_materials_to_supply = get_missing_materials(&self.data.receipe.input, &self.data.input);

                    if let Some((materials_to_supply, amount)) = maybe_materials_to_supply {
                        info!(
                            "{} is missing {} {} to start producing",
                            shop_id, materials_to_supply, amount
                        );

                        info!("{} only has {}", shop_id, &self.data.input);
                        shared::supply_command(worker, self.structure.pos, world, amount, materials_to_supply, shop_id);
                        continue;
                    }

                    shared::produce_command(worker, &self.data.receipe, shop_id);
                }
            }
        }
    }
}

fn get_missing_materials(
    receipe_input: &Inventory,
    current_store: &Inventory,
) -> Option<(InventoryItem, f32)> {
    for (&key, &item) in receipe_input.iter() {
        if current_store.get(&key) < item {
            return Some((key, item));
        }
    }
    None
}
