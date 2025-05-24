use std::fmt::Display;

use super::inventory::Inventory;

#[derive(Clone)]
pub struct Receipe {
    pub input: Inventory,
    pub output: Inventory, //maybe production can produce multiple things???
    pub requirement: f32,
}

impl Display for Receipe {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} => {} in {}", self.input, self.output, self.requirement)?;
        Ok(())
    }
}
