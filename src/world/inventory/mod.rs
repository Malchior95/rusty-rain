use std::collections::HashMap;
use std::fmt::Display;

use crate::config::inventory::InventoryItems;

pub struct Inventory {
    pub inv: HashMap<InventoryItems, f32>,
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
        T: IntoIterator<Item = (InventoryItems, f32)>,
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
        T: IntoIterator<Item = (InventoryItems, f32)>,
    {
        for (key, item) in to_add {
            self.add(&key, item);
        }
    }

    pub fn remove_range<T>(
        &mut self,
        to_remove: T,
    ) where
        T: IntoIterator<Item = (InventoryItems, f32)>,
    {
        for (key, item) in to_remove {
            self.remove(&key, item);
        }
    }

    pub fn get(
        &self,
        item: &InventoryItems,
    ) -> f32 {
        self.inv.get(item).unwrap_or(&0.0).clone()
    }

    pub fn get_mut(
        &mut self,
        item: &InventoryItems,
    ) -> &mut f32 {
        let exists = self.inv.contains_key(item);
        if !exists {
            self.inv.insert(*item, 0.0);
        }
        self.inv.get_mut(item).unwrap()
    }

    pub fn add(
        &mut self,
        item: &InventoryItems,
        amount: f32,
    ) {
        let current_amount = self.get_mut(item);
        *current_amount += amount;
    }

    pub fn remove(
        &mut self,
        item: &InventoryItems,
        amount: f32,
    ) {
        let current_amount = self.get_mut(item);
        *current_amount -= amount;
    }

    pub fn drain(&mut self) -> std::collections::hash_map::Drain<InventoryItems, f32> {
        self.inv.drain()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&InventoryItems, &f32)> {
        self.inv.iter().filter(|(_, v)| **v > 0.0)
    }

    pub fn has_any_of(
        &self,
        materials: &Vec<InventoryItems>,
    ) -> bool {
        for key in materials {
            if self.get(key) >= 1.0 {
                return true;
            }
        }
        false
    }
}
