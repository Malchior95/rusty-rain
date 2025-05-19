use std::collections::HashMap;

use log::info;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{inventory::InventoryItem, structures::shop::store::Store, world_map::WorldMap},
};

use super::ActionResult;

pub struct SupplyAction {
    pub progress: f32,
    path_cost: Vec<f32>,
    pub path: Vec<Pos>,
    pub inventory: HashMap<InventoryItem, f32>,
    reached_destination: bool,
}

impl SupplyAction {
    pub fn new(
        from: Pos,
        to: Pos,
        map: &WorldMap,
        inventory_to_haul: HashMap<InventoryItem, f32>,
        store: &mut Store,
    ) -> Option<Self> {
        info!("Worker started supplying!");

        //make a reservation in the store...
        //first, check if action can be performed
        for (key, amount) in inventory_to_haul.iter() {
            if !store.can_take(key, *amount) {
                return None;
            }
        }

        let path = pathfinding::a_star(map, from, to)?;

        //if O.K. - make a reservation
        for (key, amount) in inventory_to_haul.iter() {
            store.take(*key, *amount);
        }

        Some(Self {
            progress: 0.0,
            path_cost: map.path_to_cost(&path),
            path,
            inventory: inventory_to_haul,
            reached_destination: false,
        })
    }

    pub fn process(
        &mut self,
        store_inventory: &mut HashMap<InventoryItem, f32>,
        delta: f32,
    ) -> ActionResult {
        //action requires going both ways - therefore * 2.0
        let requirement = self.path_cost.iter().sum::<f32>();
        self.progress += delta;

        if !self.reached_destination && self.progress > requirement {
            info!("Worker reached store for supplying - and is now 'taking' reserved items!");
            self.reached_destination = true;
        }

        if self.progress > 2.0 * requirement {
            info!("Worker came back to building from supplying!");
            for (key, items) in self.inventory.drain() {
                let own_items = store_inventory.get(&key).unwrap_or(&0.0);
                store_inventory.insert(key, *own_items + items);
            }
            ActionResult::Completed
        } else {
            ActionResult::InProgress
        }
    }

    pub fn worker_position(&self) -> Pos {
        let mut acc = 0.0;
        let mut max_i = 0;

        let full_path: Vec<&Pos> = self.path.iter().chain(self.path.iter().rev()).collect();

        while acc < self.progress && max_i < full_path.len() {
            acc += self.path_cost[max_i];
            max_i += 1;
        }

        return full_path[max_i].clone();
    }
}
