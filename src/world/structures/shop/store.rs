use std::collections::HashMap;

use log::info;

use crate::{
    math::Pos,
    world::{
        World,
        inventory::InventoryItem,
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        world_map::TileType,
    },
};

pub struct Store {
    pub inventory: HashMap<InventoryItem, f32>,
}

impl Store {
    pub fn build(
        world: &mut World,
        pos: Pos,
    ) -> bool {
        if !world.map.can_build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT) {
            return false;
        }

        let woodcutter = Self { inventory: HashMap::new() };

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

        world.map.build(pos.x, pos.y, Self::WIDTH, Self::HEIGHT, || {
            TileType::Structure(ShopTypeDiscriminants::MainStore)
        });
        return true;
    }

    pub fn add(
        &mut self,
        item: InventoryItem,
        amount: f32,
    ) {
        let current = self.inventory.get(&item).unwrap_or(&0.0);

        info!("Added {} to the store. Current amount: {}", item, *current + amount);
        self.inventory.insert(item, *current + amount);
    }

    pub fn can_take(
        &self,
        item: &InventoryItem,
        amount: f32,
    ) -> bool {
        let current = self.inventory.get(item).unwrap_or(&0.0);
        *current > amount
    }

    pub fn take(
        &mut self,
        item: InventoryItem,
        amount: f32,
    ) {
        let current = self.inventory.get(&item).unwrap_or(&0.0);
        if !self.can_take(&item, amount) {
            panic!();
        }

        info!("Removed {} from the store. Current amount: {}", item, *current - amount);

        self.inventory.insert(item, *current - amount);
    }

    //pub fn process(
    //    &mut self,
    //    structure: &Structure,
    //    map: &mut WorldMap,
    //    shops: &LinkedList<Shop>,
    //    delta: f32,
    //) {
    //    //TODO: store does not need processing for now
    //}

    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 3;
}
