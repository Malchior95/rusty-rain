use std::collections::LinkedList;

pub mod shop;
use shop::{gatherer::Gatherer, hearth::Hearth, store::Store};
use strum_macros::{EnumDiscriminants, EnumIs};

use crate::{math::Pos, world::world_map::WorldMap};

pub struct Shop {
    //pub workers: Vec<Worker>,
    //pub inventory: Inventory,
    pub structure: Structure,
    pub shop_type: ShopType,
}

pub struct Structure {
    pub pos: Pos,

    pub height: u8,
    pub width: u8,
}

#[derive(EnumDiscriminants, EnumIs)]
pub enum ShopType {
    MainHearth(Hearth),
    MainStore(Store),
    Gatherer(Gatherer),
}

impl Shop {
    pub fn process(
        &mut self,
        map: &mut WorldMap,
        shops: &mut LinkedList<Shop>,
        delta: f32,
    ) {
        match self.shop_type {
            ShopType::MainHearth(ref mut hearth) => hearth.process(&self.structure, map, shops, delta),
            ShopType::Gatherer(ref mut gatherer) => gatherer.process(&self.structure, map, shops, delta),
            _ => {} //currently no update necessary...
        }
    }
}
