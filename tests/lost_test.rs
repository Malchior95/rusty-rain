use log::info;
use rusty_rain::{
    config::{buildings::Buildings, inventory::InventoryItems},
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::Inventory,
        structures::Building,
        workers::{LostAction, Worker, worker_states::WorkerWithAction},
        world_map::{TileType, resources::ResourceType},
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
    while seconds < 2.1 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    {
        let hearth = world.get_hearths().nth(0).unwrap();
        let tender = hearth.0.workers.front().unwrap();
        let b = tender.break_progress();
        info!("{} break progress", b.progress);

        //by the end of this test, woodcutter should have taken a break, and a number of fuel should
        //have been burned in the hearth
        assert!(b.progress > Worker::TIME_TO_BREAK);
        //TODO: currently worker will never become exhausted, because they will never try to find a
        //hearth (and fail). Should they become exhausted when lost?
        assert!(!tender.exhausted());
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
        let hearth = world.get_hearths().nth(0).unwrap();
        let tender = hearth.0.workers.front().unwrap();
        let b = tender.break_progress();
        info!("{} break progress", b.progress);

        //by the end of this test, woodcutter should have taken a break, and a number of fuel should
        //have been burned in the hearth
        assert!(b.progress < Worker::TIME_TO_BREAK);
        assert!(!matches!(tender, Worker::Lost(_)));
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
        building_base
            .workers
            .push_front(Worker::Lost(WorkerWithAction::<LostAction> {
                name: "Lost in the Woods".to_string(),
                inventory: Inventory::limited(5.0),
                pos: Pos::new(13, 13), //unlucky number...
                break_progress: BasicAction::new(Worker::TIME_TO_BREAK),
                exhausted: false,
                action_data: LostAction::new(),
            }))
    };

    let extra_trees_pos = vec![Pos::new(12, 12), Pos::new(12, 13), Pos::new(13, 12)];
    extra_trees_pos.iter().for_each(|p| {
        let tile = world.map.get_mut(&p);
        *tile = ResourceType::tile_tree();
    });

    let maybe_store = commands::build(world, Buildings::MainStore, Pos::new(4, 3), BuildMethod::SpawnExisting);
    if let Some(Building { building_base, .. }) = maybe_store {
        building_base.output.add(&InventoryItems::Wood, 40.0);
    }
}
