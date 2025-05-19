use crate::math::Pos;
use std::{array::from_fn, collections::LinkedList};

use inventory::InventoryItem;
use structures::{
    Shop, ShopType,
    shop::{hearth::Hearth, store::Store, woodcutter::Woodcutter},
};
use workers::Worker;
use world_map::{TileType, WorldMap};

pub mod actions;
pub mod inventory;
pub mod structures;
pub mod workers;
pub mod world_map;

pub struct World {
    pub map: WorldMap,
    pub shops: LinkedList<Shop>,
    pub unassigned_workers: LinkedList<Worker>,
}

impl World {
    pub fn new_test(
        width: usize,
        height: usize,
    ) -> Self {
        let mut map = WorldMap::new_test(width, height);

        //this is cleaver! Note that 5 in type annotations is an array size!
        let unassigned_workers = LinkedList::from(from_fn::<Worker, 5, _>(|_| Worker::default()));

        //draw road
        (3..7).map(|y| Pos::new(3, y)).for_each(|p| map.map[p.y][p.x] = TileType::Road);
        (3..8).map(|x| Pos::new(x, 6)).for_each(|p| map.map[p.y][p.x] = TileType::Road);

        let mut world = World {
            map,
            shops: LinkedList::new(),
            unassigned_workers,
        };

        let built = Hearth::build(&mut world, Pos::new(width / 2, height / 2));

        if built {
            if let ShopType::MainHearth(hearth) = &mut world.shops.back_mut().unwrap().shop_type {
                let worker = world.unassigned_workers.pop_back().unwrap();

                hearth.assign_worker(worker);
            }
        }

        let built = Woodcutter::build(&mut world, Pos::new(11, 4));

        if built {
            if let ShopType::Woodcutter(woodcutter) = &mut world.shops.back_mut().unwrap().shop_type {
                let worker = world.unassigned_workers.pop_back().unwrap();
                woodcutter.assign_worker(worker);
                woodcutter.inventory.output.insert(InventoryItem::Wood, 10.0);
            }
        }

        let built = Store::build(&mut world, Pos::new(4, 3));

        if built {
            if let ShopType::MainStore(store) = &mut world.shops.back_mut().unwrap().shop_type {
                store.inventory.insert(InventoryItem::Wood, 30.0);
            }
        }

        world
    }
}

impl World {
    pub fn next_tick(
        &mut self,
        delta: f32,
    ) {
        //when processing shops, I cannot just pass the list of all shops to shop, as that would
        //contain double reference to the same object (which is not allowed in rust)
        //I need to pop item from the queue first, and can then safely pass list of all rmaining
        //shops to it's process function. I then place the shop back in the queue
        for _ in 0..self.shops.len() {
            let mut shop = self.shops.pop_front().unwrap();
            shop.process(&mut self.map, &mut self.shops, delta);
            self.shops.push_back(shop);
        }
    }
}
