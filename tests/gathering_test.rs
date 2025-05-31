use std::io::Write;

use rusty_rain::{
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::{Inventory, InventoryItem},
        structures::builders,
        workers::{Idle, Worker, worker_with_action::WorkerWithAction},
    },
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

    let stores = world.get_stores();
    let store = stores.first().unwrap();

    //by the end of this test, some wood should be gathered and transfered to the store.

    assert!(store.output.total_items() > 0.0);
}

pub fn configure_world_for_gathering_testing(world: &mut World) {
    let maybe_store = builders::build_mainstore(world, Pos::new(4, 3));
    if let Some(store) = maybe_store {
        store.output.add(&InventoryItem::Wood, 0.0);
    }

    let maybe_woodcutter = builders::build_woodcutter(world, Pos::new(11, 5));

    if let Some(woodcutter) = maybe_woodcutter {
        woodcutter.workers.push_back(Worker::Idle(WorkerWithAction::<Idle> {
            name: "Woodchuck Chuck".to_string(),
            inventory: Inventory::limited(5.0),
            pos: woodcutter.structure.pos,
            break_progress: BasicAction::new(120.0),
            exhausted: false,
            action_data: Idle(),
        }));
    };
}
