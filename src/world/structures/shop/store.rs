use crate::{
    math::Pos,
    world::{
        World,
        inventory::Inventory,
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        world_map::TileType,
    },
};

pub struct Store {}

pub fn build_store(
    world: &mut World,
    pos: Pos,
) -> Option<&mut Shop<Store>> {
    if !world.map.can_build(pos.x, pos.y, Store::WIDTH, Store::HEIGHT) {
        return None;
    }

    let store = Store {};

    let structure = Structure {
        pos,
        height: Store::HEIGHT,
        width: Store::WIDTH,
    };

    let shop = Shop {
        structure,
        workers: Vec::with_capacity(Store::MAX_WORKERS as usize),
        max_workers: Store::MAX_WORKERS,
        output: Inventory::new(),
        data: store,
    };

    world.shops.push_back(ShopType::MainStore(shop));

    world.map.build(pos.x, pos.y, Store::WIDTH, Store::HEIGHT, || {
        TileType::Structure(ShopTypeDiscriminants::MainStore)
    });

    if let ShopType::MainStore(store) = world.shops.back_mut().unwrap() {
        return Some(store);
    }
    panic!();
}

impl Store {
    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 3;

    //TODO: for now no workers - maybe in the future have dedicated workers to hauling
    pub const MAX_WORKERS: u8 = 0;
}
