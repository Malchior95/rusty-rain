use std::collections::HashSet;

use log::info;

use crate::{
    config::{
        inventory::InventoryItems,
        receipes::{ProducedReceipe, Receipe},
    },
    world::{
        World,
        inventory::Inventory,
        structures::BuildingBase,
        workers::{Worker, worker::WorkerActionResult},
    },
};

use super::shared;

pub struct ProducerBehaviour {
    pub input: Inventory,
    pub production_cycle: usize,
    pub internal_state: InternalProducerState,
}

pub enum InternalProducerState {
    Supplying,
    Producing,
    Storing,
}

impl Default for ProducerBehaviour {
    fn default() -> Self {
        Self {
            input: Inventory::new(),
            production_cycle: 0,
            internal_state: InternalProducerState::Supplying,
        }
    }
}

impl ProducerBehaviour {
    pub fn process(
        &mut self,
        shop_base: &mut BuildingBase,
        world: &mut World,
        delta: f32,
    ) {
        let shop_id = &format!("{}", shop_base.building);
        for _ in 0..shop_base.workers.len() {
            let worker = shop_base.workers.pop_front().unwrap();

            let (mut worker, result) = worker.continue_action(shop_base.pos, delta, world, false);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::BroughtToShop(inventory) => {
                    self.input.add_range(inventory);
                }

                WorkerActionResult::ProductionComplete(receipe) => {
                    shop_base.output.add_range(receipe.output);
                }

                WorkerActionResult::Idle => {
                    worker = handle_idle(self, shop_base, worker, world, shop_id);
                }
            }
            shop_base.workers.push_back(worker);
        }
    }
}

fn handle_idle(
    pb: &mut ProducerBehaviour,
    shop_base: &mut BuildingBase,
    mut worker: Worker,
    world: &mut World,
    shop_id: &String,
) -> Worker {
    if shop_base.output.is_full() || matches!(pb.internal_state, InternalProducerState::Storing) {
        pb.internal_state = InternalProducerState::Storing;

        worker = shared::store_command(worker, world, &mut shop_base.output, shop_id);
        if shop_base.output.is_empty() {
            info!(
                "{} - delivery completed - starting to store new materials.",
                shop_base.building
            );
            pb.internal_state = InternalProducerState::Supplying
        }
        return worker;
    }

    if let InternalProducerState::Supplying = pb.internal_state {
        //this will list an array of materials that worker can bring to the store. At some point,
        //this list will be empty, or there will be no materials in the world - then
        //worker will not transition to the supplying state, but remain idle.
        //If materials to supply is empty, then I will want to start producing, because the shop
        //has ALL the necessary materials - in many variants.
        //If the shop could still take some materials, but the worker has not transitioned to the
        //supplying state - it means there are no materials in the world. Then I need to check if
        //the shop has enough to start producing - and then start producing

        let materials_to_supply = get_materials_to_supply(shop_base, pb);
        if materials_to_supply.is_empty() {
            info!("{} has all the necessary materials!", shop_base.building);
            pb.internal_state = InternalProducerState::Producing;
            return worker;
        }

        worker = shared::supply_command(worker, shop_base.pos, world, &materials_to_supply, shop_id);

        if let Worker::Idle(_) = worker {
            //worker has not transitioned to the supplying state - presumably there are no more
            //materials in the world. If possible, start producing, otherwise, remain idle

            if has_enough_materials(shop_base, pb) {
                info!(
                    "{} reserves are not full, but will start producing with: {}",
                    shop_base.building, pb.input
                );
                pb.internal_state = InternalProducerState::Producing;
            }
        }

        return worker;
    }

    if let InternalProducerState::Producing = pb.internal_state {
        if !has_enough_materials(shop_base, pb) {
            //not enough materials for anything - store and then supply
            pb.internal_state = InternalProducerState::Storing;
            return worker;
        }

        //pick receipe to make - do not worry if not enough materials for that particular one -
        //will cycle in upcomming frames and start producing something else
        let receipe_count = shop_base.building.get_data().production_receipes.len();
        let receipe_variant = &shop_base.building.get_data().production_receipes[pb.production_cycle];
        pb.production_cycle = (pb.production_cycle + 1) % receipe_count;

        let produced_receipe = if let Some(pr) = make_produced_receipe_from_variant_receipe(&pb.input, receipe_variant)
        {
            pr
        } else {
            return worker;
        };

        let idle_worker = if let Worker::Idle(idle_worker) = worker {
            idle_worker
        } else {
            return worker; //unreachable - but remain idle
        };

        info!(
            "{} (worker {}) is producing {}.",
            shop_id, idle_worker.name, produced_receipe
        );

        for (item, amount) in &produced_receipe.input {
            pb.input.remove(item, *amount);
        }

        return idle_worker.to_producing(produced_receipe);
    }

    return worker;
}

fn has_enough_materials(
    shop_base: &BuildingBase,
    pb: &ProducerBehaviour,
) -> bool {
    let receipes = &shop_base.building.get_data().production_receipes;
    for receipe in receipes {
        let mut can_make_receipe = true;
        for slot in &receipe.input {
            can_make_receipe &= has_any_of(slot, &pb.input);
        }
        if can_make_receipe {
            return true;
        }
    }

    false
}

fn get_materials_to_supply(
    shop_base: &mut BuildingBase,
    pb: &mut ProducerBehaviour,
) -> Vec<InventoryItems> {
    let mut ret = HashSet::<InventoryItems>::new();
    let receipes = &shop_base.building.get_data().production_receipes;
    for receipe in receipes {
        for slot in &receipe.input {
            for item in slot {
                let prefered_amount = f32::max(2.0 * item.1, 10.0);
                if pb.input.get(&item.0) < prefered_amount {
                    ret.insert(item.0);
                }
            }
        }
    }
    ret.drain().collect()
}

fn has_any_of(
    receipe_variant_input: &Vec<(InventoryItems, f32)>,
    current_store: &Inventory,
) -> bool {
    for (key, item) in receipe_variant_input {
        if current_store.get(key) >= *item {
            return true;
        }
    }
    false
}

///Will return none if store does not have enough materials for the receipe
fn make_produced_receipe_from_variant_receipe(
    store_inventory: &Inventory,
    receipe: &Receipe,
) -> Option<ProducedReceipe> {
    let mut pr = ProducedReceipe {
        input: Vec::new(),
        output: receipe.output.clone(),
        time_requirement: receipe.time_requirement,
    };

    for slot in &receipe.input {
        for (key, amount) in slot {
            if store_inventory.get(key) >= *amount {
                pr.input.push((*key, *amount));
                break;
            }
        }
    }
    if pr.input.len() != receipe.input.len() {
        return None;
    }
    Some(pr)
}
