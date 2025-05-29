use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use strum_macros::Display;

#[derive(Hash, PartialEq, Eq, Copy, Clone, Display)]
pub enum InventoryItem {
    Wood,
    Resin,
    Berries,
    Herbs,
    Plank, //TODO: more
}

pub type InventoryItems = (InventoryItem, f32);

pub struct Inventory {
    inv: HashMap<InventoryItem, f32>,
    pub limit: f32,
}

impl Display for Inventory {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for (key, item) in self.inv.iter() {
            write!(f, "{} {} ", key, item)?;
        }
        Ok(())
    }
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            inv: HashMap::new(),
            limit: 0.0,
        }
    }

    pub fn limited(limit: f32) -> Self {
        Self {
            inv: HashMap::new(),
            limit,
        }
    }

    pub fn is_full(&self) -> bool {
        if self.limit <= 0.0 {
            false
        } else {
            self.inv.values().map(|x| *x).sum::<f32>() >= self.limit
        }
    }

    pub fn is_empty(&self) -> bool {
        for (_, &amount) in self.inv.iter() {
            if amount > 0.0 {
                return false;
            }
        }
        true
    }

    pub fn total_items(&self) -> f32 {
        let mut acc = 0.0;
        for (_, &amount) in self.inv.iter() {
            acc += amount;
        }
        acc
    }

    pub fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = InventoryItems>,
    {
        Self {
            inv: HashMap::from_iter(iter),
            limit: 0.0,
        }
    }

    pub fn add_range<T>(
        &mut self,
        to_add: T,
    ) where
        T: IntoIterator<Item = InventoryItems>,
    {
        for (key, item) in to_add {
            self.add(&key, item);
        }
    }

    pub fn remove_range<T>(
        &mut self,
        to_remove: T,
    ) where
        T: IntoIterator<Item = InventoryItems>,
    {
        for (key, item) in to_remove {
            self.remove(&key, item);
        }
    }

    pub fn get(
        &self,
        item: &InventoryItem,
    ) -> f32 {
        self.inv.get(item).unwrap_or(&0.0).clone()
    }

    pub fn get_mut(
        &mut self,
        item: &InventoryItem,
    ) -> &mut f32 {
        let exists = self.inv.contains_key(item);
        if !exists {
            self.inv.insert(*item, 0.0);
        }
        self.inv.get_mut(item).unwrap()
    }

    pub fn add(
        &mut self,
        item: &InventoryItem,
        amount: f32,
    ) {
        let current_amount = self.get_mut(item);
        *current_amount += amount;
    }

    pub fn remove(
        &mut self,
        item: &InventoryItem,
        amount: f32,
    ) {
        let current_amount = self.get_mut(item);
        *current_amount -= amount;
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<InventoryItem, f32> {
        self.inv.drain()
    }

    pub fn clear(&mut self) {
        self.inv.clear();
    }

    pub fn transfer_until_full(
        &mut self,
        target: &mut Inventory,
    ) {
        if target.limit <= 0.0 || self.total_items() < target.limit - target.total_items() {
            for (key, items) in self.drain() {
                target.add(&key, items);
            }
        } else {
            for (&key, items) in self.inv.iter_mut() {
                let remaining_capacity = target.limit - target.total_items();
                if remaining_capacity <= 0.0 {
                    return;
                }
                let to_transfer = f32::min(remaining_capacity, *items);
                *items -= to_transfer;
                target.add(&key, to_transfer);
            }
        }
    }
}
