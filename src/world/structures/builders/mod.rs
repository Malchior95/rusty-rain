use std::collections::LinkedList;

use strum::IntoDiscriminant;

use crate::{
    math::Pos,
    world::{
        World,
        inventory::{Inventory, InventoryItem},
        receipes::Receipe,
        structures::shop::{
            hearth::{Hearth, HearthAction},
            store::Store,
        },
        world_map::{TileType, resources::ResourceType},
    },
};

use super::{
    Shop, ShopType, Structure,
    shop::{gatherer::Gatherer, producer::Producer},
};

pub fn build<'a, T, F>(
    world: &'a mut World,
    structure: Structure,
    max_workers: u8,
    data: T,
    shop_type_wrap: F,
) where
    F: FnOnce(Shop<T>) -> ShopType,
{
    if !world.map.can_build(&structure) {
        return;
    }

    let shop = Shop {
        structure,
        workers: LinkedList::new(),
        max_workers,
        output: Inventory::limited(10.0),
        data,
    };

    let shop_type = shop_type_wrap(shop);

    world.map.build(&shop_type.get_non_generic().structure, || {
        TileType::Structure(shop_type.discriminant())
    });

    world.shops.push_back(shop_type);
}

pub fn build_herbalist<'a>(
    world: &'a mut World,
    pos: Pos,
) -> Option<&'a mut Shop<Gatherer>> {
    build(
        world,
        Structure {
            pos,
            width: Gatherer::WIDTH,
            height: Gatherer::HEIGHT,
        },
        Gatherer::MAX_WORKERS,
        Gatherer {
            resource_type: ResourceType::Berries,
            storing_all: false,
        },
        |s| ShopType::Gatherer(s),
    );

    if let ShopType::Gatherer(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    unreachable!();
}

pub fn build_woodcutter<'a>(
    world: &'a mut World,
    pos: Pos,
) -> Option<&'a mut Shop<Gatherer>> {
    build(
        world,
        Structure {
            pos,
            width: Gatherer::WIDTH,
            height: Gatherer::HEIGHT,
        },
        3,
        Gatherer {
            resource_type: ResourceType::Tree,
            storing_all: false,
        },
        |s| ShopType::Gatherer(s),
    );

    if let ShopType::Gatherer(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    unreachable!();
}

pub fn build_lumbermill<'a>(
    world: &'a mut World,
    pos: Pos,
) -> Option<&'a mut Shop<Producer>> {
    build(
        world,
        Structure {
            pos,
            width: 2,
            height: 3,
        },
        2,
        Producer {
            storing_all: false,
            receipe: Receipe {
                input: vec![(InventoryItem::Wood, 2.0)],
                output: vec![(InventoryItem::Plank, 3.0)],
                requirement: 30.0,
            },
            input: Inventory::new(),
        },
        |s| ShopType::Producer(s),
    );

    if let ShopType::Producer(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    unreachable!();
}

pub fn build_hearth<'a>(
    world: &'a mut World,
    pos: Pos,
) -> Option<&'a mut Shop<Hearth>> {
    build(
        world,
        Structure {
            pos,
            width: Hearth::WIDTH,
            height: Hearth::HEIGHT,
        },
        Hearth::MAX_WORKERS,
        Hearth {
            action: HearthAction::Idle,
            inventory: Inventory::from_iter([(InventoryItem::Wood, 20.0)]),
        },
        |s| ShopType::MainHearth(s),
    );

    if let ShopType::MainHearth(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    unreachable!();
}

pub fn build_mainstore<'a>(
    world: &'a mut World,
    pos: Pos,
) -> Option<&'a mut Shop<Store>> {
    build(
        world,
        Structure {
            pos,
            width: 4,
            height: 3,
        },
        0,
        Store {},
        |s| ShopType::MainStore(s),
    );

    if let ShopType::MainStore(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    unreachable!();
}
