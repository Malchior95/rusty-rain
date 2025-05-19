use std::{collections::HashMap, default};

use log::info;

use crate::math::Pos;

use super::{
    inventory::{self, InventoryItem},
    world_map::WorldMap,
};

//I just had Rust implicilty copy this, without putting it back... so the actual value in enum was
//never updated. Need to be careful with Copy
pub struct BasicAction {
    pub progress: f32,
    pub requirement: f32,
}

#[derive(Default)]
pub enum ActionResult {
    #[default]
    InProgress,
    Completed,
}

#[derive(Default)]
pub enum HaulActionResult {
    #[default]
    InProgress,
    Completed(HashMap<InventoryItem, f32>),
}

impl BasicAction {
    pub fn new(requirement: f32) -> Self {
        Self {
            progress: 0.0,
            requirement,
        }
    }

    pub fn process(&mut self, delta: f32) -> ActionResult {
        self.progress += delta;
        if self.progress > self.requirement {
            ActionResult::Completed
        } else {
            ActionResult::InProgress
        }
    }
}

pub struct HaulAction {
    pub progress: f32,
    path_cost: Vec<f32>,
    pub path: Vec<Pos>,
    pub inventory: HashMap<InventoryItem, f32>,
    reached_destination: bool,
}

pub struct GatherResourceAction {
    pub progress: f32,
    pub requirement: f32,
}

impl HaulAction {
    pub fn new(path: Vec<Pos>, map: &WorldMap, inventory: HashMap<InventoryItem, f32>) -> Self {
        Self {
            progress: 0.0,
            path_cost: map.path_to_cost(&path),
            path,
            inventory,
            reached_destination: false,
        }
    }

    pub fn process(&mut self, delta: f32) -> ActionResult {
        //action requires going both ways - therefore * 2.0
        let requirement = self.path_cost.iter().sum::<f32>();
        self.progress += delta;

        if !self.reached_destination && self.progress > requirement {
            info!("Worker reached store!");
            self.reached_destination = true;
        }

        if self.progress > 2.0 * requirement {
            info!("Worker came back to building!");
            //can I do this without cloning? e.g. by consuming the struct?
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
