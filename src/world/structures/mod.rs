pub mod builders;
pub mod shop;
use impl_variant_non_generic::{ImplVariantNonGeneric, IntoNonGeneric};
use shop::{gatherer::Gatherer, hearth::Hearth, producer::Producer, store::Store};
use strum_macros::{Display, EnumDiscriminants, EnumIs};

use crate::math::Pos;

use super::{World, inventory::Inventory, workers::Worker};

#[derive(IntoNonGeneric)]
pub struct Shop<T> {
    pub structure: Structure,
    pub workers: Vec<Worker>,
    pub max_workers: u8,
    pub output: Inventory, //todo: really needed here? maybe move to data?
    pub data: T,
}

pub struct Structure {
    pub pos: Pos,

    pub height: u8,
    pub width: u8,
}

#[derive(EnumDiscriminants, EnumIs)]
#[strum_discriminants(derive(Display))]
#[derive(ImplVariantNonGeneric)]
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
}
