use rusty_rain::{
    config::{buildings::Buildings, inventory::InventoryItems},
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::Inventory,
        structures::Building,
        workers::{Idle, Worker, worker_with_action::WorkerWithAction},
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

    configure_world_for_hearth_testing(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 3.5 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    let hearth = world.get_hearths().nth(0).unwrap();
    let tender = hearth.0.workers.front().unwrap();
    let b = tender.get_non_generic().break_progress;

    //by the end of this test, woodcutter should have taken a break, and a number of fuel should
    //have been burned in the hearth
    assert!(b.progress < 3.0 * 60.0);
    assert!(!tender.get_non_generic().exhausted);
    assert!(hearth.1.input.total_items() < 15.0);
}

pub fn configure_world_for_hearth_testing(world: &mut World) {
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
}
