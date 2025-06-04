use rusty_rain::{
    config::{buildings::Buildings, inventory::InventoryItems},
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        building::{Building, BuildingBehaviour},
        inventory::Inventory,
        worker::{Idle, Worker, worker_states::WorkerWithAction},
    },
    world_interaction::commands::{self, BuildMethod},
};
use std::io::Write;

use rusty_rain::FRAME_NUM;
use std::sync::atomic::Ordering;

pub fn test(mut world: World) {
    let _ = env_logger::builder()
        .format(|buf, record| {
            let tick_num = FRAME_NUM.load(Ordering::Relaxed);
            writeln!(buf, "@{}\t{}", tick_num, record.args())?;
            Ok(())
        })
        .try_init();

    configure_world(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 3.5 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    let _lumbermill = world
        .shops
        .iter()
        .find_map(|s| {
            if let BuildingBehaviour::Producer(shop) = &s.building_behaviour {
                Some(shop)
            } else {
                None
            }
        })
        .unwrap();

    let store = world.get_stores().nth(0).unwrap();

    assert!(store.0.output.get(&InventoryItems::Bricks) <= 0.0);
    assert!(store.0.output.get(&InventoryItems::Fabric) <= 0.0);
}

pub fn configure_world(world: &mut World) {
    let maybe_hearth = commands::build(
        world,
        Buildings::MainHearth,
        Pos::new(world.map.width() / 2, world.map.height() / 2),
        BuildMethod::SpawnExisting,
    );

    if let Some(Building {
        building_base,
        building_behaviour: BuildingBehaviour::Hearth(hearth),
    }) = maybe_hearth
    {
        hearth
            .unassigned_workers
            .push_front(Worker::Idle(WorkerWithAction::<Idle> {
                name: "Bob".to_string(),
                inventory: Inventory::limited(5.0),
                pos: building_base.pos,
                break_progress: BasicAction::new(120.0),
                exhausted: false,
                action_data: Idle(),
            }))
    };

    let maybe_store = commands::build(world, Buildings::MainStore, Pos::new(4, 3), BuildMethod::SpawnExisting);
    if let Some(Building { building_base, .. }) = maybe_store {
        building_base.output.add(&InventoryItems::Bricks, 2.0);
        building_base.output.add(&InventoryItems::Fabric, 2.0);
    } else {
        panic!();
    }

    let maybe_build_zone_lumbermill = commands::build(
        world,
        Buildings::Lumbermill,
        Pos::new(4, 8),
        BuildMethod::SpawnBuildZone,
    );

    if maybe_build_zone_lumbermill.is_none() {
        panic!();
    }
}
