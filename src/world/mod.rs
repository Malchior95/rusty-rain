use crate::{FRAME_NUM, math::Pos};
use std::{array::from_fn, collections::LinkedList, sync::atomic::Ordering};

use inventory::InventoryItem;
use structures::{
    Shop, ShopType,
    shop::{gatherer::Gatherer, hearth::Hearth, store::Store},
};
use workers::Worker;
use world_map::{TileType, WorldMap, resources::ResourceType};

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
    //TODO: remove this eventually - move main to a separate create and keep tests in tests
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

        *map.get_mut(&Pos::new(3, 12)) = ResourceType::tile_berry();

        *map.get_mut(&Pos::new(3, 7)) = ResourceType::tile_tree();

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

        let built = Gatherer::build(&mut world, Pos::new(11, 4), ResourceType::Tree);

        if built {
            if let ShopType::Gatherer(woodcutter) = &mut world.shops.back_mut().unwrap().shop_type {
                let worker = world.unassigned_workers.pop_back().unwrap();
                woodcutter.assign_worker(worker);

                let worker = world.unassigned_workers.pop_back().unwrap();
                woodcutter.assign_worker(worker);

                let worker = world.unassigned_workers.pop_back().unwrap();
                woodcutter.assign_worker(worker);
            }
        }

        let built = Store::build(&mut world, Pos::new(4, 3));

        if built {
            if let ShopType::MainStore(store) = &mut world.shops.back_mut().unwrap().shop_type {
                store.inventory.add(InventoryItem::Wood, 50.0);
            }
        }

        let built = Gatherer::build(&mut world, Pos::new(8, 3), ResourceType::Berries);

        if built {
            if let ShopType::Gatherer(herbalist) = &mut world.shops.back_mut().unwrap().shop_type {
                let worker = world.unassigned_workers.pop_back().unwrap();
                herbalist.assign_worker(worker);
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
        //
        //I just learned I could use something like RefCell, to check borrowing rules at runtime,
        //if I need to. I dont wanna. They say that in programming you either write a code or write
        //a theorem. I'm in the second team
        for _ in 0..self.shops.len() {
            let mut shop = self.shops.pop_front().unwrap();
            shop.process(self, delta);
            self.shops.push_back(shop);
        }

        FRAME_NUM.fetch_add(1, Ordering::Relaxed);
    }
}
