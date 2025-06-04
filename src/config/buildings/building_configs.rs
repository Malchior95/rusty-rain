use std::sync::LazyLock;

use crate::{
    config::{inventory::InventoryItems, receipes::receipes_config},
    world::structures::BuildingBehaviourDiscriminants,
};

use super::BuildingConfig;

pub static WOODCUTTER: LazyLock<BuildingConfig> = LazyLock::new(|| BuildingConfig {
    name: "Woodcutter's Camp",
    max_workers: 3,
    production_receipes: Vec::new(),
    gathered_resource_types: vec![InventoryItems::Wood],
    width: 2,
    height: 2,
    building_behaviour: BuildingBehaviourDiscriminants::Gatherer,
    build_time: 15.0,
    build_materials: vec![(InventoryItems::Wood, 10.0), (InventoryItems::Parts, 2.0)],
});

pub static LUMBERMILL: LazyLock<BuildingConfig> = LazyLock::new(|| BuildingConfig {
    name: "Lumber Mill",
    max_workers: 2,
    production_receipes: vec![
        &receipes_config::PLANKS_3,
        &receipes_config::PACK_OF_TRADE_GOODS_1,
        &receipes_config::SCROLLS_1,
    ],
    gathered_resource_types: Vec::new(),
    width: 2,
    height: 3,
    building_behaviour: BuildingBehaviourDiscriminants::Producer,
    build_time: 25.0,
    build_materials: vec![(InventoryItems::Bricks, 2.0), (InventoryItems::Fabric, 2.0)],
});

pub static MAIN_STORE: LazyLock<BuildingConfig> = LazyLock::new(|| BuildingConfig {
    name: "Main Store",
    max_workers: 0,
    production_receipes: Vec::new(),
    gathered_resource_types: Vec::new(),
    width: 4,
    height: 3,
    building_behaviour: BuildingBehaviourDiscriminants::Store,
    //this is free!
    build_time: 0.0,
    build_materials: Vec::new(),
});

pub static MAIN_HEARTH: LazyLock<BuildingConfig> = LazyLock::new(|| BuildingConfig {
    name: "Main Hearth",
    max_workers: 0,
    production_receipes: Vec::new(),
    gathered_resource_types: Vec::new(),
    width: 4,
    height: 4,
    building_behaviour: BuildingBehaviourDiscriminants::Hearth,
    //this is free!
    build_time: 0.0,
    build_materials: Vec::new(),
});
