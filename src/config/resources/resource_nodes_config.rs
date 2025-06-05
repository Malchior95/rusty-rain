use std::sync::LazyLock;

use crate::config::inventory::InventoryItems;

use super::{NodeSize, ResourceNodeConfig};

pub static DEWBERRY_BUSH: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Dewberry Bush",
    primary_resource: InventoryItems::Berries,
    bonus_resources: vec![(InventoryItems::Berries, 0.2)],
    total_charges: 15.0,
    size: NodeSize::Small,
});

pub static DEWBERRY_BUSH_LARGE: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Large Dewberry Bush",
    primary_resource: InventoryItems::Berries,
    bonus_resources: vec![(InventoryItems::Berries, 0.8)],
    total_charges: 70.0,
    size: NodeSize::Large,
});

pub static CLAY_DEPOSIT: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Clay Deposit",
    primary_resource: InventoryItems::Clay,
    bonus_resources: vec![(InventoryItems::CopperOre, 0.5), (InventoryItems::Roots, 0.2)],
    total_charges: 20.0,
    size: NodeSize::Small,
});

pub static CLAY_DEPOSIT_LARGE: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Clay Deposit Large",
    primary_resource: InventoryItems::Clay,
    bonus_resources: vec![(InventoryItems::CopperOre, 0.75), (InventoryItems::Roots, 0.3)],
    total_charges: 60.0,
    size: NodeSize::Large,
});

pub static BLEEDING_TOOTH: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Bleedig Tooth",
    primary_resource: InventoryItems::Mushrooms,
    bonus_resources: vec![(InventoryItems::Insects, 0.2)],
    total_charges: 20.0,
    size: NodeSize::Large,
});

pub static BLEEDING_TOOTH_LARGE: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Bleedig Tooth Large",
    primary_resource: InventoryItems::Mushrooms,
    bonus_resources: vec![(InventoryItems::Insects, 0.4)],
    total_charges: 70.0,
    size: NodeSize::Small,
});

pub static DRIZZLEWING_NEST: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Drizzlewing Nest",
    primary_resource: InventoryItems::Eggs,
    bonus_resources: vec![(InventoryItems::Meat, 0.2)],
    total_charges: 15.0,
    size: NodeSize::Large,
});

pub static DRIZZLEWING_NEST_LARGE: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Drizzlewing Nest Large",
    primary_resource: InventoryItems::Eggs,
    bonus_resources: vec![(InventoryItems::Meat, 0.4)],
    total_charges: 70.0,
    size: NodeSize::Small,
});

pub static FLAX_FIELD: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Flax Field",
    primary_resource: InventoryItems::PlantFiber,
    bonus_resources: vec![(InventoryItems::Clay, 0.25), (InventoryItems::Insects, 0.2)],
    total_charges: 20.0,
    size: NodeSize::Large,
});

pub static FLAX_FIELD_LARGE: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Flax Field Large",
    primary_resource: InventoryItems::PlantFiber,
    bonus_resources: vec![(InventoryItems::Clay, 0.5), (InventoryItems::Insects, 0.3)],
    total_charges: 60.0,
    size: NodeSize::Small,
});

pub static GRASSCAP_MUSHROOMS: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Grasscap Mushrooms",
    primary_resource: InventoryItems::Mushrooms,
    bonus_resources: vec![(InventoryItems::Insects, 0.2)],
    total_charges: 20.0,
    size: NodeSize::Large,
});

pub static GRASSCAP_MUSHROOMS_LARGE: LazyLock<ResourceNodeConfig> = LazyLock::new(|| ResourceNodeConfig {
    name: "Grasscap Mushrooms Large",
    primary_resource: InventoryItems::Mushrooms,
    bonus_resources: vec![(InventoryItems::Insects, 0.4)],
    total_charges: 70.0,
    size: NodeSize::Small,
});
