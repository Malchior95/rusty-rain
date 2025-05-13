use crate::world::structures::hearth::Hearth;
use crate::world::structures::shop::Shop;
use std::{array::from_fn, collections::LinkedList};

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

        let main_hearth = Hearth {
            x: width / 2,
            y: height / 2,
        };

        map.build(main_hearth.x, main_hearth.y, Hearth::WIDTH, Hearth::HEIGHT);

        //this is cleaver! Note that 5 in type annotations is an array size!
        let unassigned_workers = LinkedList::from(from_fn::<Worker, 5, _>(|_| Worker::default()));

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

        world
    }
}

impl World {
    pub fn next_tick(&self, delta: f32) -> World {
        //TODO: for now I only 'clone' everything, to avoid substituting in-place.
        //I do not yet know if calculating values in-place has significant drawbacks
        //For now the cost of cloning is negligible...
        //I will want to replace all those clones with "calculate state"
        //I do not yet know if I would like to pass an array of commands ("user actions") to this
        //function, or rather implement this elsewhere. Then this function would only progress all
        //action already in progress and continue game's state as if no user interaction happened
        World {
            map: self.map.clone(),
            shops: self.shops.clone(),
            main_hearth: self.main_hearth.clone(),
            unassigned_workers: self.unassigned_workers.clone(),
        }
    }
}
