use log::info;
use rusty_rain::{
    config::{buildings::Buildings, inventory::InventoryItems},
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        inventory::Inventory,
        structures::{Building, BuildingBase, BuildingBehaviour, StoreBehaviour},
        workers::{Idle, Worker, worker_with_action::WorkerWithAction},
    },
    world_interaction::commands::{self, BuildMethod},
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
    while seconds < 3.6 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    {
        let lumbermill = world
            .shops
            .iter()
            .filter_map(|s| {
                if let BuildingBehaviour::Producer(p) = &s.building_behaviour {
                    Some((&s.building_base, p))
                } else {
                    None
                }
            })
            .nth(0)
            .unwrap();

        let store = world.get_stores().nth(0).unwrap();

        info!("store has: {}", store.0.output);
        info!("lumbermill has: {}, input: {}", lumbermill.0.output, lumbermill.1.input);
    }

    {
        let store = get_stores_mut(&mut world).nth(0).unwrap();

        //by the end of this test, some wood should be taken from the store, some planks produced and
        //brought to the store

        assert!(store.0.output.get(&InventoryItems::Plank) == 10.0);
        assert!(store.0.output.get(&InventoryItems::Wood) == 0.0);

        //now try adding materials for other production receipes!
        store.0.output.add(&InventoryItems::Barrels, 6.0);
        store.0.output.add(&InventoryItems::Leather, 4.0);
        store.0.output.add(&InventoryItems::Wine, 3.0);
    }
    info!("=====================================================================================================\n\n");

    {
        while seconds < 7.4 * 60.0 {
            world.next_tick(DELTA);
            seconds += DELTA;
        }
    }
    let lumbermill = world
        .shops
        .iter()
        .filter_map(|s| {
            if let BuildingBehaviour::Producer(p) = &s.building_behaviour {
                Some((&s.building_base, p))
            } else {
                None
            }
        })
        .nth(0)
        .unwrap();

    let store = world.get_stores().nth(0).unwrap();
    info!("store has: {}", store.0.output);
    info!("lumbermill has: {}, input: {}", lumbermill.0.output, lumbermill.1.input);

    assert!(store.0.output.get(&InventoryItems::PackOfTradeGoods) == 2.0);
    assert!(store.0.output.get(&InventoryItems::Scrolls) == 8.0);
}

fn configure_world_for_production_testing(world: &mut World) {
    let maybe_store = commands::build(world, Buildings::MainStore, Pos::new(4, 3), BuildMethod::SpawnExisting);
    if let Some(Building { building_base, .. }) = maybe_store {
        building_base.output.add(&InventoryItems::Wood, 15.0);
    }

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

fn get_stores_mut(world: &mut World) -> impl Iterator<Item = (&mut BuildingBase, &mut StoreBehaviour)> {
    world.shops.iter_mut().filter_map(|s| {
        if let BuildingBehaviour::Store(h) = &mut s.building_behaviour {
            Some((&mut s.building_base, h))
        } else {
            None
        }
    })
}
