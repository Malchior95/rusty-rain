pub mod builders;
pub mod shop;
use std::collections::LinkedList;

//use enum_dispatch::enum_dispatch;
use impl_variant_non_generic::{ImplVariantNonGeneric, IntoNonGeneric};
use shop::{gatherer::Gatherer, hearth::Hearth, producer::Producer, store::Store};
use strum_macros::{Display, EnumDiscriminants, EnumIs};

use crate::math::Pos;

use super::{World, inventory::Inventory, workers::Worker};

//TODO: when my arcane macro fails me in the future, I could use something like enum_dispatch to
//expose enum data. It requires more boiler plate, but presumably works
//
//#[enum_dispatch]
//pub trait ShopBase {
//    fn structure(&self) -> &Structure;
//    fn workers(&self) -> &LinkedList<Worker>;
//    fn output(&self) -> &Inventory;
//}
//
//impl<T> ShopBase for Shop<T> {
//    fn structure(&self) -> &Structure {
//        &self.structure
//    }
//    fn workers(&self) -> &LinkedList<Worker> {
//        &self.workers
//    }
//    fn output(&self) -> &Inventory {
//        &self.output
//    }
//}

#[derive(IntoNonGeneric)]
pub struct Shop<T> {
    pub structure: Structure,
    pub workers: LinkedList<Worker>,
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
//#[enum_dispatch(ShopBase)]
pub enum ShopType {
    MainHearth(Shop<Hearth>),
    MainStore(Shop<Store>),
    Gatherer(Shop<Gatherer>),
    Producer(Shop<Producer>),
}
