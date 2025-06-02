use std::collections::HashMap;

use super::{inventory::InventoryItem, receipes::Receipe};

pub struct Producer {
    pub name: String,
    pub max_workers: u8,
    pub receipes: Vec<Receipe>,
    pub width: u8,
    pub height: u8,
}

pub struct Gatherer {
    pub name: String,
    pub max_workers: u8,
    pub resource_types: Vec<InventoryItem>,
    pub width: u8,
    pub height: u8,
}

pub enum BuiltInGathererBuildings {
    //Gatherer
    Woodcutter,
    Herbalist,
    Stonecutter,
    Harvester,
    Forager,
    Trapper,
}

pub enum BuiltInProducerBuildings {
    //essential
    MainHearth,
    MainStore,

    //Producer - materials
    CrudeWorkstation,
    MakeshiftPost,
    Lumbermill,
    Kiln,
    Brickyard,

    //Producer - food
    FieldKitchen,
    Butcher,
}

fn a() {
    let hm: HashMap<&InventoryItem, f32> = HashMap::new();
}
