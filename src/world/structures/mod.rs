pub mod builders;
pub mod shop;
use shop::{gatherer::Gatherer, hearth::Hearth, producer::Producer, store::Store};
use strum_macros::{Display, EnumDiscriminants, EnumIs};

use crate::math::Pos;

use super::{World, inventory::Inventory, workers::Worker};

pub struct Shop<T> {
    //pub inventory: Inventory,
    pub structure: Structure,
    pub workers: Vec<Worker>,
    pub max_workers: u8,
    pub output: Inventory, //todo: really needed here? maybe move to data?
    pub data: T,
}

#[derive(Clone)]
pub struct Structure {
    pub pos: Pos,

    pub height: u8,
    pub width: u8,
}

#[derive(EnumDiscriminants, EnumIs)]
#[strum_discriminants(derive(Display))]
pub enum ShopType {
    MainHearth(Shop<Hearth>),
    MainStore(Shop<Store>),
    Gatherer(Shop<Gatherer>),
    Producer(Shop<Producer>),
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
            ShopType::Producer(producer) => producer.process(world, delta),
            _ => {} //currently no update necessary...
        }
    }

    pub fn location(&self) -> Pos {
        match self {
            ShopType::MainHearth(shop) => shop.structure.pos,
            ShopType::MainStore(shop) => shop.structure.pos,
            ShopType::Gatherer(shop) => shop.structure.pos,
            ShopType::Producer(shop) => shop.structure.pos,
        }
    }

    pub fn inventory(&self) -> &Inventory {
        match self {
            ShopType::MainHearth(shop) => &shop.output,
            ShopType::MainStore(shop) => &shop.output,
            ShopType::Gatherer(shop) => &shop.output,
            ShopType::Producer(shop) => &shop.output,
        }
    }

    pub fn inventory_mut(&mut self) -> &mut Inventory {
        match self {
            ShopType::MainHearth(shop) => &mut shop.output,
            ShopType::MainStore(shop) => &mut shop.output,
            ShopType::Gatherer(shop) => &mut shop.output,
            ShopType::Producer(shop) => &mut shop.output,
        }
    }
}
