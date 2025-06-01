use rusty_rain::{
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::{Inventory, InventoryItem},
        structures::{ShopType, builders},
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
            if let ShopType::Producer(shop) = s {
                Some(shop)
            } else {
                None
            }
        })
        .unwrap();

    let store = world.get_stores().nth(0).unwrap();

    assert!(store.into_non_generic().output.get(&InventoryItem::Wood) <= 30.0);
}

pub fn configure_world(world: &mut World) {
    let maybe_hearth = builders::build_hearth(world, Pos::new(world.map.width() / 2, world.map.height() / 2));
    if let Some(hearth) = maybe_hearth {
        hearth
            .data
            .unassigned_workers
            .push_front(Worker::Idle(WorkerWithAction::<Idle> {
                name: "Bob".to_string(),
                inventory: Inventory::limited(5.0),
                pos: hearth.structure.pos,
                break_progress: BasicAction::new(120.0),
                exhausted: false,
                action_data: Idle(),
            }))
    };

    let maybe_store = builders::build_mainstore(world, Pos::new(4, 3));
    if let Some(store) = maybe_store {
        store.output.add(&InventoryItem::Wood, 40.0);
    }

    commands::build_lumbermill(world, Pos::new(4, 8), BuildMethod::SpawnExisting);
}
