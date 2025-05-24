use crate::{FRAME_NUM, math::Pos};
use std::{array::from_fn, collections::LinkedList, sync::atomic::Ordering};

use actions::BasicAction;
use inventory::{Inventory, InventoryItem};
use structures::{
    Shop, ShopType,
    shop::{
        gatherer,
        hearth::{self, Hearth},
        store::{self, Store},
    },
};
use workers::{Idle, Worker, WorkerWithAction};
use world_map::{TileType, WorldMap, resources::ResourceType};

pub mod actions;
pub mod inventory;
pub mod structures;
pub mod workers;
pub mod world_map;

pub struct World {
    pub map: WorldMap,
    pub shops: LinkedList<ShopType>,
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
        //let unassigned_workers = LinkedList::from(from_fn::<Worker, 5, _>(|_| Worker::default()));

        //draw road
        (3..7)
            .map(|y| Pos::new(3, y))
            .for_each(|p| map.map[p.y][p.x] = TileType::Road);
        (3..8)
            .map(|x| Pos::new(x, 6))
            .for_each(|p| map.map[p.y][p.x] = TileType::Road);

        *map.get_mut(&Pos::new(3, 12)) = ResourceType::tile_berry();

        *map.get_mut(&Pos::new(3, 7)) = ResourceType::tile_tree();

        let mut world = World {
            map,
            shops: LinkedList::new(),
            unassigned_workers: LinkedList::new(),
        };

        let maybe_hearth = hearth::build_hearth(&mut world, Pos::new(width / 2, height / 2));
        if let Some(hearth) = maybe_hearth {
            hearth.workers.push(Worker::Idle(WorkerWithAction::<Idle> {
                name: "Hearth tender".to_string(),
                inventory: Inventory::limited(5.0),
                pos: hearth.structure.pos,
                break_progress: BasicAction::new(120.0),
                action_data: Idle(),
            }));

            hearth.data.inventory.add(InventoryItem::Wood, 100.0);
        };

        let maybe_store = store::build_store(&mut world, Pos::new(4, 3));
        if let Some(store) = maybe_store {
            store.output.add(InventoryItem::Wood, 100.0);
        }

        let maybe_woodcutter = gatherer::build_gatherer(&mut world, Pos::new(11, 5), ResourceType::Tree);

        if let Some(woodcutter) = maybe_woodcutter {
            woodcutter.workers.push(Worker::Idle(WorkerWithAction::<Idle> {
                name: "Woodchuck Chuck".to_string(),
                inventory: Inventory::limited(5.0),
                pos: woodcutter.structure.pos,
                break_progress: BasicAction::new(120.0),
                action_data: Idle(),
            }));
        };

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
