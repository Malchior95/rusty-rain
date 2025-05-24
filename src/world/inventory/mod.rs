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

#[derive(Clone)]
pub struct Inventory {
    inv: HashMap<InventoryItem, f32>,
    pub limit: f32,
}

impl Display for Inventory {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for (key, item) in self.iter() {
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
        for (_, &amount) in self.iter() {
            acc += amount;
        }
        acc
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

    pub fn add_range(
        &mut self,
        to_add: &Inventory,
    ) {
        for (&key, &item) in to_add.iter() {
            self.add(key, item);
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

    pub fn remove_range(
        &mut self,
        to_remove: &Inventory,
    ) {
        for (&key, &amount) in to_remove.iter() {
            self.remove(key, amount);
        }
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<InventoryItem, f32> {
        self.inv.drain()
    }

    pub fn extract(&mut self) -> Inventory {
        Inventory::from_iter(self.drain())
    }

    pub fn transfer_until_full(
        &mut self,
        target: &mut Inventory,
    ) {
        if target.limit <= 0.0 || self.total_items() < target.limit - target.total_items() {
            for (key, items) in self.drain() {
                target.add(key, items);
            }
        } else {
            for (&key, items) in self.inv.iter_mut() {
                let remaining_capacity = target.limit - target.total_items();
                if remaining_capacity <= 0.0 {
                    return;
                }
                let to_transfer = f32::min(remaining_capacity, *items);
                *items -= to_transfer;
                target.add(key, to_transfer);
            }
        }
    }

    pub fn has_at_least(
        &self,
        other: &Inventory,
    ) -> bool {
        for (key, item) in other.iter() {
            if self.get(&key) < *item {
                return false;
            }
        }
        true
    }
}
