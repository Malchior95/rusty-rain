use std::collections::HashMap;

use crate::world::{World, actions::Action, inventory::Inventory, workers::Worker};

pub mod woodcutter;

#[derive(Clone)]
pub struct Shop {
    pub workers: Vec<Worker>,
    pub inventory: Inventory,

    pub x: usize,
    pub y: usize,

    pub height: u8,
    pub width: u8,

    pub shop_type: ShopType,
}

#[derive(Clone)]
pub enum ShopType {
    Woodcutter,
    Herbalist,
    //TODO: more
}

impl Shop {
    pub fn assign_worker(&mut self, mut worker: Worker) {
        worker.action = Action::Idle;

        self.workers.push(worker);
    }
}
