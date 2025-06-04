use std::fmt::Display;

use super::inventory::InventoryItems;
pub mod receipes_config;

pub struct Receipe {
    pub input: Vec<Vec<(InventoryItems, f32)>>,
    pub output: Vec<(InventoryItems, f32)>,
    pub time_requirement: f32,
    pub receipe_level: ReceipeLevel,
}

pub struct ProducedReceipe {
    pub input: Vec<(InventoryItems, f32)>,
    pub output: Vec<(InventoryItems, f32)>,
    pub time_requirement: f32,
}

impl Display for ProducedReceipe {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        for (key, item) in &self.input {
            write!(f, "{} {} ", key, item)?;
        }
        write!(f, "=> ")?;

        for (key, item) in &self.output {
            write!(f, "{} {} ", key, item)?;
        }
        write!(f, "t: {}", self.time_requirement)?;
        Ok(())
    }
}

pub enum ReceipeLevel {
    Crude,
    Basic,
    Regular,
    Specialized,
    Legendary,
}
