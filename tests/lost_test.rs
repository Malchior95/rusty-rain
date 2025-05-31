use log::info;
use rusty_rain::{
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::{Inventory, InventoryItem},
        structures::builders,
        workers::{LostAction, Worker, worker_with_action::WorkerWithAction},
        world_map::{TileType, resources::ResourceType},
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

    configure_world(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 2.1 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    {
        let hs = world.get_hearths();
        let hearth = hs.first().unwrap();
        let tender = hearth.workers.front().unwrap();
        let b = tender.get_non_generic().break_progress;
        info!("{} break progress", b.progress);

        //by the end of this test, woodcutter should have taken a break, and a number of fuel should
        //have been burned in the hearth
        assert!(b.progress > Worker::TIME_TO_BREAK);
        //TODO: currently worker will never become exhausted, because they will never try to find a
        //hearth (and fail). Should they become exhausted when lost?
        assert!(!tender.get_non_generic().exhausted);
        assert!(matches!(tender, Worker::Lost(_)));

        let tile = world.map.get_mut(&Pos::new(13, 12));
        *tile = TileType::Empty;
    }

    let mut seconds = 0.0;
    while seconds < 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    {
        let hs = world.get_hearths();
        let hearth = hs.first().unwrap();
        let tender = hearth.workers.front().unwrap();
        let b = tender.get_non_generic().break_progress;
        info!("{} break progress", b.progress);

        //by the end of this test, woodcutter should have taken a break, and a number of fuel should
        //have been burned in the hearth
        assert!(b.progress < Worker::TIME_TO_BREAK);
        assert!(!matches!(tender, Worker::Lost(_)));
    }
}

pub fn configure_world(world: &mut World) {
    let maybe_hearth = builders::build_hearth(world, Pos::new(world.map.width() / 2, world.map.height() / 2));
    if let Some(hearth) = maybe_hearth {
        hearth.workers.push_back(Worker::Lost(WorkerWithAction::<LostAction> {
            name: "Lost in the Woods".to_string(),
            inventory: Inventory::limited(5.0),
            pos: Pos::new(13, 13), //unlucky number...
            break_progress: BasicAction::new(Worker::TIME_TO_BREAK),
            exhausted: false,
            action_data: LostAction::new(),
        }));
    };

    let extra_trees_pos = vec![Pos::new(12, 12), Pos::new(12, 13), Pos::new(13, 12)];
    extra_trees_pos.iter().for_each(|p| {
        let tile = world.map.get_mut(&p);
        *tile = ResourceType::tile_tree();
    });

    let maybe_store = builders::build_mainstore(world, Pos::new(4, 3));
    if let Some(store) = maybe_store {
        store.output.add(&InventoryItem::Wood, 5.0);
    }
}
