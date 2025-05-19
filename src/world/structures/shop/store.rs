use std::{
    collections::{HashMap, LinkedList},
    default,
};

use crate::{
    math::Pos,
    world::{
        World,
        actions::{GatherResourceAction, HaulAction},
        inventory::{Inventory, InventoryItem},
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::Worker,
        world_map::{TileType, WorldMap},
    },
};

pub struct Store {
    pub inventory: HashMap<InventoryItem, f32>,
}

pub struct StoreWorker {
    pub worker: Worker,
}

#[derive(Default)]
pub enum StoreWorkerAction {
    #[default]
    Idle,
    Haul(HaulAction),
    GatherResouce(GatherResourceAction),
}

impl Store {
    pub fn build(world: &mut World, pos: Pos) -> bool {
        if !world.map.can_build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT) {
            return false;
        }

        let woodcutter = Self {
            inventory: HashMap::new(),
        };

        //FIXME: check if enterance is accessible...

        let structure = Structure {
            pos,
            height: Self::HEIGHT,
            width: Self::WIDTH,
            enterance: Pos::new(pos.x, pos.y - 1),
        };

        let shop = Shop {
            structure,
            shop_type: ShopType::MainStore(woodcutter),
        };

        world.shops.push_back(shop);

        world
            .map
            .build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT, || {
                TileType::Structure(ShopTypeDiscriminants::MainStore)
            });
        return true;
    }

    pub fn process(
        &mut self,
        structure: &Structure,
        map: &mut WorldMap,
        shops: &LinkedList<Shop>,
        delta: f32,
    ) {
        //TODO:
    }

    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 3;
}
