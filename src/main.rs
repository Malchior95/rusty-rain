use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

use log::info;
use world::World;

mod ai;
mod data_helpers;
mod math;
mod world;

static FRAME_NUM: AtomicUsize = AtomicUsize::new(0);

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "@{}\t{}", FRAME_NUM.load(Ordering::Relaxed), record.args())?;
            Ok(())
        })
        .init();

    info!("Initializing test world");
    let mut world = World::new_test(16, 16);
    info!("\n{}", world.map);

    //run simulation for 30s (or equivalent)

    let timer = Instant::now();

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 120.0 {
        FRAME_NUM.fetch_add(1, Ordering::Relaxed);

        world.next_tick(DELTA);
        seconds += DELTA;
    }

    info!("Simulation took: {} ms", timer.elapsed().as_micros() as f32 / 1e3);
    println!("Simulation took: {} ms", timer.elapsed().as_micros() as f32 / 1e3);
}
