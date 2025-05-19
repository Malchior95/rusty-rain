use std::collections::LinkedList;

pub mod shop;
use shop::{hearth::Hearth, store::Store, woodcutter::Woodcutter};
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

    //TODO: I do not want to implement a pathfinding algorithm that will be able to enter
    //impassable fields (e.g. structure) only if this is the field it is going to. That would
    //require me to flood-fill that structure and remove it from the pathfinding. For now, let me
    //implement a field 'door' that lies outside of the building and does not register on the map.
    //That building will be possible to be entered through the door. What if structure cannot be
    //entered? E.g. it is a garden, a gate, or is otherwise traversible? Shh... do not worry about
    //this. Maybe one tiles always needs to be designeted? e.g. for building
    //TODO: if I ever decide to introduce map layers, enterance could be marked on an additional
    //layer (cannot really mark it on base layer, as it will interfere with e.g. paths)
    pub enterance: Pos,
}

#[derive(EnumDiscriminants, EnumIs)]
pub enum ShopType {
    MainHearth(Hearth),
    Woodcutter(Woodcutter),
    MainStore(Store),
}

impl Shop {
    pub(crate) fn process(
        &mut self,
        map: &mut WorldMap,
        shops: &mut LinkedList<Shop>,
        delta: f32,
    ) {
        match self.shop_type {
            ShopType::MainHearth(ref mut hearth) => hearth.process(&self.structure, map, shops, delta),
            ShopType::Woodcutter(ref mut woodcutter) => woodcutter.process(&self.structure, map, shops, delta),
            ShopType::MainStore(_) => {} //currently no update necessary...
        }
    }
}
