use std::collections::HashMap;

use crate::world::{
    World,
    inventory::Inventory,
    structures::shop::{Shop, ShopType},
    world_map::TileType,
};

pub fn build_store(world: &mut World, x: usize, y: usize) -> bool {
    const HEIGHT: u8 = 2;
    const WIDTH: u8 = 2;

    if !world.map.can_build(x, y, WIDTH, HEIGHT) {
        return false;
    }

    let store = Shop {
        workers: Vec::new(),
        inventory: Inventory {
            input: HashMap::new(),
            output: HashMap::new(),
            output_limit: f32::MAX,
        },
        x,
        y,
        height: HEIGHT,
        width: WIDTH,
        shop_type: ShopType::Store,
    };

    world
        .map
        .build(x, y, WIDTH, HEIGHT, TileType::Structure(ShopType::Store));

    world.shops.push_back(store);

    true
}
