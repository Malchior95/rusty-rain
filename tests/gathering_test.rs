use std::io::Write;

use rusty_rain::{
    config::{buildings::Buildings, inventory::InventoryItems},
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        building::Building,
        inventory::Inventory,
        worker::{Idle, Worker, worker_states::WorkerWithAction},
    },
    world_interaction::commands::{self, BuildMethod},
};

use std::sync::atomic::Ordering;

use rusty_rain::FRAME_NUM;

pub fn test(mut world: World) {
    let _ = env_logger::builder()
        .format(|buf, record| {
            let tick_num = FRAME_NUM.load(Ordering::Relaxed);
            writeln!(buf, "@{}\t{}", tick_num, record.args())?;
            Ok(())
        })
        .try_init();

    configure_world_for_gathering_testing(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 4.0 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    let store = world.get_stores().nth(0).unwrap();

    //by the end of this test, some wood should be gathered and transfered to the store.

    assert!(store.0.output.total_items() > 0.0);
}

pub fn configure_world_for_gathering_testing(world: &mut World) {
    let maybe_store = commands::build(world, Buildings::MainStore, Pos::new(4, 3), BuildMethod::SpawnExisting);
    if let Some(Building { building_base, .. }) = maybe_store {
        building_base.output.add(&InventoryItems::Wood, 0.0);
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
}
