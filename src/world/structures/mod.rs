use std::collections::LinkedList;

use shop::Shop;

use super::world_map::WorldMap;

pub mod hearth;
pub mod shop;

pub trait Structure {
    fn process(&mut self, map: &mut WorldMap, shops: &LinkedList<Shop>, delta: f32);
}
