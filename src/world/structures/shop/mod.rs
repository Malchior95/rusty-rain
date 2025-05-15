use std::collections::LinkedList;

use crate::world::{inventory::Inventory, workers::Worker, world_map::WorldMap};

pub mod store;
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

//TODO: make shop type enum with data, have shared properties in Shop, and custom logic in
//ShopType, e.g. ShopType(MainHearth)

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ShopType {
    Woodcutter,
    Herbalist,
    Store, //TODO: more
}

impl Shop {
    pub fn assign_worker(&mut self, worker: Worker) {
        self.workers.push(worker);
    }

    pub(crate) fn process(&mut self, world: &mut WorldMap, shops: &LinkedList<Shop>, delta: f32) {
        //TODO:
    }
}
