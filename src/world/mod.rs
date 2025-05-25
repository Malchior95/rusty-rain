use std::collections::LinkedList;

use structures::ShopType;
use workers::Worker;
use world_map::WorldMap;

use crate::FRAME_NUM;

pub mod actions;
pub mod inventory;
pub mod receipes;
pub mod structures;
pub mod workers;
pub mod world_map;

pub struct World {
    pub map: WorldMap,
    pub shops: LinkedList<ShopType>,
    pub unassigned_workers: LinkedList<Worker>,
    pub frame_number: usize,
}

impl World {
    pub fn next_tick(
        &mut self,
        delta: f32,
    ) {
        FRAME_NUM.store(self.frame_number, std::sync::atomic::Ordering::Relaxed);
        //when processing shops, I cannot just pass the list of all shops to shop, as that would
        //contain double reference to the same object (which is not allowed in rust)
        //I need to pop item from the queue first, and can then safely pass list of all rmaining
        //shops to it's process function. I then place the shop back in the queue
        //
        //I just learned I could use something like RefCell, to check borrowing rules at runtime,
        //if I need to. I dont wanna. They say that in programming you either write a code or write
        //a theorem. I'm in the second team
        for _ in 0..self.shops.len() {
            let mut shop = self.shops.pop_front().unwrap();
            shop.process(self, delta);
            self.shops.push_back(shop);
        }

        self.frame_number += 1;
    }
}
