use std::collections::LinkedList;

use crate::{
    math::Pos,
    world::{
        World,
        inventory::{Inventory, InventoryItem},
        receipes::Receipe,
        structures::{
            Shop, ShopType, ShopTypeDiscriminants, Structure, build_zone::BuildZone, shop::producer::Producer,
        },
        world_map::TileType,
    },
};

pub enum BuildMethod {
    SpawnExisting,
    SpawnBuildZone,
}

pub fn build_lumbermill(
    world: &mut World,
    pos: Pos,
    build_method: BuildMethod,
) -> bool {
    let structure = Structure {
        pos,
        height: 3,
        width: 2,
    };

    if !world.map.can_build(&structure) {
        return false;
    }

    world
        .map
        .build(&structure, || TileType::BuildZone(ShopTypeDiscriminants::Producer));

    let shop_type = ShopType::Producer(Shop::<Producer> {
        structure,
        workers: LinkedList::new(),
        max_workers: 2,
        output: Inventory::limited(10.0),
        data: Producer {
            storing_all: false,
            receipe: Receipe {
                input: vec![(InventoryItem::Wood, 2.0)],
                output: vec![(InventoryItem::Plank, 3.0)],
                requirement: 30.0,
            },
            input: Inventory::new(),
        },
    });

    match build_method {
        BuildMethod::SpawnExisting => {
            world.shops.push_back(shop_type);
        }
        BuildMethod::SpawnBuildZone => {
            let build_zone = BuildZone::new(shop_type);
            world.build_zones.push_back(build_zone);
        }
    }

    true
}
