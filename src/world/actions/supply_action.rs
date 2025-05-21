use log::info;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{inventory::Inventory, structures::shop::store::Store, world_map::WorldMap},
};

use super::ActionResult;

pub struct SupplyAction {
    pub progress: f32,
    path_cost: Vec<f32>,
    pub path: Vec<Pos>,
    pub inventory: Inventory,
    reached_destination: bool,
}

impl SupplyAction {
    pub fn new(
        from: Pos,
        to: Pos,
        map: &WorldMap,
        inventory_to_haul: Inventory,
        store: &mut Store,
    ) -> Option<Self> {
        info!("Worker started supplying!");

        //make a reservation in the store...
        //first, check if action can be performed
        for (key, amount) in inventory_to_haul.iter() {
            if store.inventory.get(key) < *amount {
                return None;
            }
        }

        let path = pathfinding::a_star(map, from, to)?;

        //if O.K. - make a reservation
        for (key, amount) in inventory_to_haul.iter() {
            store.inventory.remove(*key, *amount);
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
        store_inventory: &mut Inventory,
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
                info!("--- delivering {} {}", key, items);
                store_inventory.add(key, items);
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
