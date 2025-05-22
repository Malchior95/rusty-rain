//use std::array::from_fn;
//use std::collections::LinkedList;
//use std::io::Write;
//
//use std::{sync::atomic::Ordering, time::Instant};
//
//use log::info;
//use rusty_rain::math::Pos;
//use rusty_rain::world::inventory::InventoryItem;
//use rusty_rain::world::structures::ShopType;
//use rusty_rain::world::structures::shop::hearth::Hearth;
//use rusty_rain::world::structures::shop::store::Store;
//use rusty_rain::world::structures::shop::woodcutter::Woodcutter;
//use rusty_rain::world::workers::Worker;
//use rusty_rain::world::world_map::{TileType, WorldMap};
//use rusty_rain::{FRAME_NUM, world::World};
//
//#[cfg(test)]
//#[test]
//fn test_world() {
//    env_logger::builder()
//        .format(|buf, record| {
//            writeln!(buf, "@{}\t{}", FRAME_NUM.load(Ordering::Relaxed), record.args())?;
//            Ok(())
//        })
//        .init();
//
//    info!("Initializing test world");
//    let mut world = new_test(16, 16);
//    info!("\n{}", world.map);
//
//    //run simulation for 120.0s
//
//    let timer = Instant::now();
//
//    let mut seconds = 0.0;
//    const DELTA: f32 = 1.0 / 30.0;
//    while seconds < 120.0 {
//        world.next_tick(DELTA);
//        seconds += DELTA;
//    }
//
//    info!("Simulation took: {} ms", timer.elapsed().as_micros() as f32 / 1e3);
//    println!("Simulation took: {} ms", timer.elapsed().as_micros() as f32 / 1e3);
//}
//
//pub fn new_test(
//    width: usize,
//    height: usize,
//) -> World {
//    let mut map = WorldMap::new_test(width, height);
//
//    //this is cleaver! Note that 5 in type annotations is an array size!
//    let unassigned_workers = LinkedList::from(from_fn::<Worker, 5, _>(|_| Worker::default()));
//
//    //draw road
//    (3..7).map(|y| Pos::new(3, y)).for_each(|p| map.map[p.y][p.x] = TileType::Road);
//    (3..8).map(|x| Pos::new(x, 6)).for_each(|p| map.map[p.y][p.x] = TileType::Road);
//
//    let mut world = World {
//        map,
//        shops: LinkedList::new(),
//        unassigned_workers,
//    };
//
//    let built = Hearth::build(&mut world, Pos::new(width / 2, height / 2));
//
//    if built {
//        if let ShopType::MainHearth(hearth) = &mut world.shops.back_mut().unwrap().shop_type {
//            let worker = world.unassigned_workers.pop_back().unwrap();
//
//            hearth.assign_worker(worker);
//        }
//    }
//
//    let built = Woodcutter::build(&mut world, Pos::new(11, 4));
//
//    if built {
//        if let ShopType::Woodcutter(woodcutter) = &mut world.shops.back_mut().unwrap().shop_type {
//            let worker = world.unassigned_workers.pop_back().unwrap();
//            woodcutter.assign_worker(worker);
//            woodcutter.inventory.output.add(InventoryItem::Wood, 10.0);
//        }
//    }
//
//    let built = Store::build(&mut world, Pos::new(4, 3));
//
//    if built {
//        if let ShopType::MainStore(store) = &mut world.shops.back_mut().unwrap().shop_type {
//            store.inventory.add(InventoryItem::Wood, 30.0);
//        }
//    }
//
//    world
//}
