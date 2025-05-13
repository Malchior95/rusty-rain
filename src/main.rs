#![allow(dead_code, unused_variables)]
use ai::a_star::debug_path_drawer::PathDrawer;
use log::info;
use math::Pos;
//TODO: remove the code above
use world::World;

mod ai;
mod data_helpers;
mod math;
mod world;

fn main() {
    env_logger::init();

    info!("Initializing test world");
    let world = World::new_test(16, 16);
    info!("\n{}", world.map);

    //ai test
    let start = Pos { x: 12, y: 12 };
    let end = Pos { x: 4, y: 3 };
    let path = crate::ai::a_star::a_star(&world.map, start, end);

    if let Some(path) = path {
        let map_drawer = PathDrawer {
            map: &world.map,
            path: &path,
        };

        info!("\n{}", map_drawer);
    };

    let next_world = world.next_tick(1.0 / 30.0);
}
