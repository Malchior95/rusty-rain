use std::io::Write;
use std::sync::atomic::Ordering;
use std::time::Instant;

use log::info;
use rusty_rain::FRAME_NUM;
use rusty_rain::world::World;

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

    //run simulation

    let timer = Instant::now();

    let mut seconds = 0.0;
    const DELTA: f32 = 1.0 / 30.0;
    while seconds < 10.0 * 60.0 {
        world.next_tick(DELTA);
        seconds += DELTA;
    }

    info!("Simulation took: {} ms", timer.elapsed().as_micros() as f32 / 1e3);
    println!("Simulation took: {} ms", timer.elapsed().as_micros() as f32 / 1e3);
}
