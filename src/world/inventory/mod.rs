use std::collections::HashMap;
use std::hash::Hash;

use strum_macros::Display;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Display)]
pub enum InventoryItem {
    Wood,
    Resin,
    Berries,
    Herbs,
    //TODO: more
}

#[derive(Clone)]
pub struct Inventory {
    inv: HashMap<InventoryItem, f32>,
    limit: f32,
}
impl Inventory {
    pub fn new() -> Self {
        Self {
            inv: HashMap::new(),
            limit: 0.0,
        }
    }
    pub fn limited(limit: f32) -> Self {
        Self { inv: HashMap::new(), limit }
    }

    pub fn is_full(&self) -> bool {
        if self.limit < 0.0 {
            true
        } else {
            self.inv.values().map(|x| *x).sum::<f32>() >= self.limit
        }
    }

    pub fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (InventoryItem, f32)>,
    {
        Self {
            inv: HashMap::from_iter(iter),
            limit: 0.0,
        }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, InventoryItem, f32> {
        self.inv.iter()
    }

    pub fn get(
        &self,
        item: &InventoryItem,
    ) -> f32 {
        self.inv.get(item).unwrap_or(&0.0).clone()
    }

    pub fn add(
        &mut self,
        item: InventoryItem,
        amount: f32,
    ) {
        let current_amount = self.get(&item);
        self.inv.insert(item, current_amount + amount);
    }

    pub fn remove(
        &mut self,
        item: InventoryItem,
        amount: f32,
    ) {
        let current_amount = self.get(&item);
        self.inv.insert(item, current_amount - amount);
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<InventoryItem, f32> {
        self.inv.drain()
    }
}
