use rusty_rain::{
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::{Inventory, InventoryItem},
        structures::builders,
        workers::{Worker, worker_with_action::Idle, worker_with_action::WorkerWithAction},
    },
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

    let hs = world.get_hearths();
    let hearth = hs.first().unwrap();
    let tender = hearth.workers.front().unwrap();
    let b = tender.get_non_generic().break_progress;

    //by the end of this test, woodcutter should have taken a break, and a number of fuel should
    //have been burned in the hearth
    assert!(b.progress < 3.0 * 60.0);
    assert!(!tender.get_non_generic().exhausted);
    assert!(hearth.data.inventory.total_items() < 15.0);
}

pub fn configure_world_for_hearth_testing(world: &mut World) {
    let maybe_hearth = builders::build_hearth(world, Pos::new(world.map.width() / 2, world.map.height() / 2));
    if let Some(hearth) = maybe_hearth {
        hearth.workers.push_back(Worker::Idle(WorkerWithAction::<Idle> {
            name: "Hearth tender".to_string(),
            inventory: Inventory::limited(5.0),
            pos: hearth.structure.pos,
            break_progress: BasicAction::new(120.0),
            exhausted: false,
            action_data: Idle(),
        }));
    };

    let maybe_store = builders::build_mainstore(world, Pos::new(4, 3));
    if let Some(store) = maybe_store {
        store.output.add(&InventoryItem::Wood, 5.0);
    }
}
