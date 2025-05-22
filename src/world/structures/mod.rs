use std::collections::LinkedList;

pub mod shop;
use shop::{gatherer::Gatherer, hearth::Hearth, store::Store};
use strum_macros::EnumDiscriminants;

use crate::{math::Pos, world::world_map::WorldMap};

use super::World;

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

#[derive(EnumDiscriminants)]
pub enum ShopType {
    MainHearth(Hearth),
    MainStore(Store),
    Gatherer(Gatherer),
}

impl Shop {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        match &mut self.shop_type {
            ShopType::MainHearth(hearth) => hearth.process(&self.structure, world, delta),
            ShopType::Gatherer(gatherer) => gatherer.process(&self.structures, world, delta),
            _ => {} //currently no update necessary...
        }
    }
}
