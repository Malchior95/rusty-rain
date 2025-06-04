use std::io::Write;
use std::{collections::LinkedList, sync::atomic::Ordering};

use rusty_rain::config::buildings::Buildings;
use rusty_rain::config::inventory::InventoryItems;
use rusty_rain::world::structures::Building;
use rusty_rain::world::workers::Idle;
use rusty_rain::world::workers::worker_states::WorkerWithAction;
use rusty_rain::world_interaction::commands::{self, BuildMethod};
use rusty_rain::{
    FRAME_NUM,
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::Inventory,
        workers::Worker,
        world_map::{TileType, WorldMap, resources::ResourceType},
    },
};

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            let tick_num = FRAME_NUM.load(Ordering::Relaxed);
            writeln!(buf, "@{}\t{}", tick_num, record.args())?;
            Ok(())
        })
        .init();

    let mut world = new_test_world(16, 16);

    configure_world(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }
}

pub fn new_test_map(
    width: usize,
    height: usize,
) -> WorldMap {
    let mut world = WorldMap::new(width, height);

    //create boundary of trees
    for y in 0..height {
        for x in 0..width {
            if [0, 1, height - 2, height - 1].contains(&y) || [0, 1, width - 2, width - 1].contains(&x) {
                world.map[y][x] = ResourceType::tile_tree();
            }
        }
    }

    world
}

pub fn new_test_world(
    width: usize,
    height: usize,
) -> World {
    let mut map = new_test_map(width, height);

    //draw road
    (3..7)
        .map(|y| Pos::new(3, y))
        .for_each(|p| map.map[p.y][p.x] = TileType::Road);
    (3..8)
        .map(|x| Pos::new(x, 6))
        .for_each(|p| map.map[p.y][p.x] = TileType::Road);

    //plant berry bush
    *map.get_mut(&Pos::new(3, 12)) = ResourceType::tile_berry();

    //plant tree
    *map.get_mut(&Pos::new(3, 7)) = ResourceType::tile_tree();

    World {
        map,
        shops: LinkedList::new(),
        frame_number: 0,
        build_zones: LinkedList::new(),
    }
}

pub fn configure_world(world: &mut World) {
    let maybe_hearth = commands::build(
        world,
        Buildings::MainHearth,
        Pos::new(world.map.width() / 2, world.map.height() / 2),
        BuildMethod::SpawnExisting,
    );

    if let Some(Building { building_base, .. }) = maybe_hearth {
        building_base.workers.push_front(Worker::Idle(WorkerWithAction::<Idle> {
            name: "Hearth Tender".to_string(),
            inventory: Inventory::limited(5.0),
            pos: building_base.pos,
            break_progress: BasicAction::new(120.0),
            exhausted: false,
            action_data: Idle(),
        }))
    };

    let maybe_store = commands::build(world, Buildings::MainStore, Pos::new(4, 3), BuildMethod::SpawnExisting);
    if let Some(Building { building_base, .. }) = maybe_store {
        building_base.output.add(&InventoryItems::Wood, 40.0);
    }

    let maybe_woodcutter = commands::build(
        world,
        Buildings::Woodcutter,
        Pos::new(11, 5),
        BuildMethod::SpawnExisting,
    );

    if let Some(Building { building_base, .. }) = maybe_woodcutter {
        building_base.workers.push_back(Worker::Idle(WorkerWithAction::<Idle> {
            name: "Woodchuck Chuck".to_string(),
            inventory: Inventory::limited(5.0),
            pos: building_base.pos,
            break_progress: BasicAction::new(120.0),
            exhausted: false,
            action_data: Idle(),
        }));
    };

    let maybe_lumbermill = commands::build(world, Buildings::Lumbermill, Pos::new(5, 9), BuildMethod::SpawnExisting);

    if let Some(Building { building_base, .. }) = maybe_lumbermill {
        building_base.workers.push_back(Worker::Idle(WorkerWithAction::<Idle> {
            name: "Jane".to_string(),
            inventory: Inventory::limited(5.0),
            pos: building_base.pos,
            break_progress: BasicAction::new(120.0),
            exhausted: false,
            action_data: Idle(),
        }));
    };
}
