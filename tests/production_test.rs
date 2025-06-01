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
use std::io::Write;

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

    configure_world_for_production_testing(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 5.0 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    let store = world.get_stores().nth(0).unwrap();

    //by the end of this test, some wood should be taken from the store, some planks produced and
    //brought to the store

    assert!(store.output.get(&InventoryItem::Plank) > 0.0);
    assert!(store.output.get(&InventoryItem::Wood) < 40.0);
}

pub fn configure_world_for_production_testing(world: &mut World) {
    let maybe_store = builders::build_mainstore(world, Pos::new(4, 3));
    if let Some(store) = maybe_store {
        store.output.add(&InventoryItem::Wood, 40.0);
    }

    let maybe_lumbermill = builders::build_lumbermill(world, Pos::new(5, 9));

    if let Some(lumbermill) = maybe_lumbermill {
        lumbermill.workers.push_back(Worker::Idle(WorkerWithAction::<Idle> {
            name: "Jane".to_string(),
            inventory: Inventory::limited(5.0),
            pos: lumbermill.structure.pos,
            break_progress: BasicAction::new(120.0),
            exhausted: false,
            action_data: Idle(),
        }));
    };
}
