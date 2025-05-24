use std::{any::Any, collections::LinkedList};

pub mod shop;
use shop::{gatherer::Gatherer, hearth::Hearth, store::Store};
use strum_macros::{Display, EnumDiscriminants};

use crate::{math::Pos, world::world_map::WorldMap};

use super::{World, inventory::Inventory, workers::Worker};

pub struct Shop<T> {
    //pub inventory: Inventory,
    pub structure: Structure,
    pub workers: Vec<Worker>,
    pub max_workers: u8,
    pub output: Inventory,
    pub data: T,
}

pub struct Structure {
    pub pos: Pos,

    pub height: u8,
    pub width: u8,
}

#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
pub enum ShopType {
    MainHearth(Shop<Hearth>),
    MainStore(Shop<Store>),
    Gatherer(Shop<Gatherer>),
}

impl ShopType {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        match self {
            ShopType::MainHearth(hearth) => hearth.process(world, delta),
            ShopType::Gatherer(gatherer) => gatherer.process(world, delta),
            _ => {} //currently no update necessary...
        }
    }

    pub fn location(&self) -> Pos {
        match self {
            ShopType::MainHearth(shop) => shop.structure.pos,
            ShopType::MainStore(shop) => shop.structure.pos,
            ShopType::Gatherer(shop) => shop.structure.pos,
        }
    }

    pub fn inventory(&self) -> &Inventory {
        match self {
            ShopType::MainHearth(shop) => &shop.output,
            ShopType::MainStore(shop) => &shop.output,
            ShopType::Gatherer(shop) => &shop.output,
        }
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        match self {
            ShopType::MainHearth(shop) => &mut shop.output,
            ShopType::MainStore(shop) => &mut shop.output,
            ShopType::Gatherer(shop) => &mut shop.output,
        }
    }
}
