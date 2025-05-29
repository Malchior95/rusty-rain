use std::fmt::Display;

use super::inventory::InventoryItems;

#[derive(Clone)]
pub struct Receipe {
    pub input: Vec<InventoryItems>,
    pub output: Vec<InventoryItems>, //maybe production can produce multiple things???
    pub requirement: f32,
}

impl Display for Receipe {
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
        write!(f, "t: {}", self.requirement)?;
        Ok(())
    }
}
