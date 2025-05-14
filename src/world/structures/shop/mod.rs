use crate::world::{inventory::Inventory, workers::Worker, world_map::WorldMap};

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
    pub fn assign_worker(&mut self, worker: Worker) {
        self.workers.push(worker);
    }

    pub(crate) fn process(&mut self, world: &mut WorldMap, delta: f32) {
        todo!()
    }
}
