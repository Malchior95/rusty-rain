use std::io::Write;

use log::info;
use std::sync::atomic::Ordering;

use rusty_rain::FRAME_NUM;
mod helpers;

#[cfg(test)]
#[test]
fn test_hearth() {
    let mut world = helpers::new_test_world(16, 16);

    let _ = env_logger::builder()
        .format(|buf, record| {
            let tick_num = FRAME_NUM.load(Ordering::Relaxed);
            writeln!(buf, "@{}\t{}", tick_num, record.args())?;
            Ok(())
        })
        .try_init();

    helpers::configure_world_for_hearth_testing(&mut world);

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

    info!("Break progress at: {}", b.progress);

    //by the end of this test, woodcutter should have taken a break, and a number of fuel should
    //have been burned in the hearth
    assert!(b.progress < 3.0 * 60.0);
    assert!(!tender.get_non_generic().exhausted);
    assert!(hearth.data.inventory.total_items() < 15.0);
}

#[cfg(test)]
#[test]
fn test_gathering() {
    let mut world = helpers::new_test_world(16, 16);

    let _ = env_logger::builder()
        .format(|buf, record| {
            let tick_num = FRAME_NUM.load(Ordering::Relaxed);
            writeln!(buf, "@{}\t{}", tick_num, record.args())?;
            Ok(())
        })
        .try_init();

    helpers::configure_world_for_gathering_testing(&mut world);

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

#[cfg(test)]
#[test]
fn test_production() {
    use rusty_rain::world::inventory::InventoryItem;

    let mut world = helpers::new_test_world(16, 16);

    let _ = env_logger::builder()
        .format(|buf, record| {
            let tick_num = FRAME_NUM.load(Ordering::Relaxed);
            writeln!(buf, "@{}\t{}", tick_num, record.args())?;
            Ok(())
        })
        .try_init();

    helpers::configure_world_for_production_testing(&mut world);

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 5.0 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    let stores = world.get_stores();
    let store = stores.first().unwrap();

    //by the end of this test, some wood should be taken from the store, some planks produced and
    //brought to the store

    assert!(store.output.get(&InventoryItem::Plank) > 0.0);
    assert!(store.output.get(&InventoryItem::Wood) < 40.0);
}
