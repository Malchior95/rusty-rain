#![allow(dead_code, unused_variables, unused_imports)]
use std::io::Write;
use std::time::Instant;

use ai::a_star::{AStarPathFinder, debug_path_drawer::PathDrawer};
use log::info;
use math::Pos;
//TODO: remove the code above
use world::World;

mod ai;
mod data_helpers;
mod math;
mod world;

static mut FRAME_NUM: u32 = 0;

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "@{}\t{}", unsafe { FRAME_NUM }, record.args())?;
            //
            Ok(())
        })
        .init();

    info!("Initializing test world");
    let mut world = World::new_test(16, 16);
    info!("\n{}", world.map);

    //ai test
    let start = Pos { x: 12, y: 12 };
    let end = Pos { x: 4, y: 3 };

    let timer = Instant::now();

    let path_finder = AStarPathFinder::new(&world.map, start, end);

    let path = path_finder.a_star();

    if let Some(path) = path {
        info!(
            "Path found in {} ms",
            timer.elapsed().as_micros() as f32 / 1e3
        );

        let map_drawer = PathDrawer {
            map: &world.map,
            path: &path,
        };

        info!("\n{}", map_drawer);
    };

    //run simulation for 30s (or equivalent)

    let timer = Instant::now();

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 120.0 {
        unsafe {
            FRAME_NUM += 1;
        }
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    info!(
        "Simulation took: {} ms",
        timer.elapsed().as_micros() as f32 / 1e3
    );
}
