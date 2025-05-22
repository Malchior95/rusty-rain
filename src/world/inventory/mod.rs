use std::collections::HashMap;
use std::hash::Hash;

use strum_macros::Display;

pub struct IOInventory {
    pub input: Inventory,
    pub output: Inventory,
    pub output_limit: f32,
}

impl Default for IOInventory {
    fn default() -> Self {
        Self {
            input: Default::default(),
            output: Default::default(),
            output_limit: 10.0,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Display)]
pub enum InventoryItem {
    Wood,
    Resin,
    Berries,
    Herbs,
    //TODO: more
}

impl IOInventory {
    pub fn is_full(&self) -> bool {
        self.output.0.values().map(|x| *x).sum::<f32>() >= self.output_limit
    }
}

pub struct Inventory(HashMap<InventoryItem, f32>);
impl Inventory {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (InventoryItem, f32)>,
    {
        Self(HashMap::from_iter(iter))
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, InventoryItem, f32> {
        self.0.iter()
    }

    pub fn get(
        &self,
        item: &InventoryItem,
    ) -> f32 {
        self.0.get(item).unwrap_or(&0.0).clone()
    }

    pub fn add(
        &mut self,
        item: InventoryItem,
        amount: f32,
    ) {
        let current_amount = self.get(&item);
        self.0.insert(item, current_amount + amount);
    }

    pub fn remove(
        &mut self,
        item: InventoryItem,
        amount: f32,
    ) {
        let current_amount = self.get(&item);
        self.0.insert(item, current_amount - amount);
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<InventoryItem, f32> {
        self.0.drain()
    }
}

impl Default for Inventory {
    fn default() -> Self {
        Self(Default::default())
    }
}
