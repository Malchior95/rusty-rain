use crate::world::structures::shop::Shop;
use crate::{math::Pos, world::structures::hearth::Hearth};
use std::{array::from_fn, collections::LinkedList};

use structures::Structure;
use structures::shop::store;
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
    pub main_hearth: Hearth,
    pub unassigned_workers: LinkedList<Worker>,
}

impl World {
    pub fn new_test(width: usize, height: usize) -> Self {
        let mut map = WorldMap::new_test(width, height);

        //this is cleaver! Note that 5 in type annotations is an array size!
        let mut unassigned_workers =
            LinkedList::from(from_fn::<Worker, 5, _>(|_| Worker::default()));
        let worker = unassigned_workers.pop_front();

        let main_hearth = Hearth::new(
            Pos {
                x: width / 2,
                y: height / 2,
            },
            worker.unwrap(),
        );

        map.build(
            main_hearth.pos.x,
            main_hearth.pos.y,
            Hearth::WIDTH,
            Hearth::HEIGHT,
            TileType::MainHearth,
        );

        let path = (3..11).map(|y| Pos::new(3, y));

        path.for_each(|p| map.map[p.y][p.x] = TileType::Road);

        let mut world = World {
            map,
            shops: LinkedList::new(),
            main_hearth,
            unassigned_workers,
        };

        let built = Shop::build_woodcutter(&mut world, 4, 4);

        if built {
            //assign two woodcutters
            if let Some(mut woodcutter) = world.shops.pop_back() {
                if let Some(worker) = world.unassigned_workers.pop_front() {
                    woodcutter.assign_worker(worker);
                }
                if let Some(worker) = world.unassigned_workers.pop_front() {
                    woodcutter.assign_worker(worker);
                }
                world.shops.push_back(woodcutter);
            };
        }

        let built = store::build_store(&mut world, 11, 3);

        world
    }
}

impl World {
    pub fn next_tick(&mut self, delta: f32) {
        //for shop in &mut self.shops {
        //    shop.process(&mut self.map, delta);
        //}

        //for worker in &mut self.unassigned_workers {
        //    worker.process(delta);
        //}

        self.main_hearth
            .process(&mut self.map, &mut self.shops, delta);

        //TODO: pop from front of the list of all shops, then invoke process with that item and the
        //reminder of the list
    }
}
