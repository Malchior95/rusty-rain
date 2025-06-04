use std::sync::LazyLock;

use crate::config::inventory::InventoryItems;

use super::{Receipe, ReceipeLevel};

pub static PLANKS_3: LazyLock<Receipe> = LazyLock::new(|| Receipe {
    input: vec![vec![(InventoryItems::Wood, 3.0)]],
    output: vec![(InventoryItems::Plank, 2.0)],
    time_requirement: 28.0,
    receipe_level: ReceipeLevel::Specialized,
});

pub static PACK_OF_TRADE_GOODS_1: LazyLock<Receipe> = LazyLock::new(|| Receipe {
    input: vec![vec![
        (InventoryItems::Dye, 8.0),
        (InventoryItems::Oil, 8.0),
        (InventoryItems::Flour, 6.0),
        (InventoryItems::Pottery, 6.0),
        (InventoryItems::Barrels, 6.0),
        (InventoryItems::Waterskins, 6.0),
    ]],
    output: vec![(InventoryItems::PackOfTradeGoods, 2.0)],
    time_requirement: 42.0,
    receipe_level: ReceipeLevel::Basic,
});

pub static SCROLLS_1: LazyLock<Receipe> = LazyLock::new(|| Receipe {
    input: vec![
        vec![
            (InventoryItems::Leather, 4.0),
            (InventoryItems::PlantFiber, 4.0),
            (InventoryItems::Wood, 10.0),
        ],
        vec![(InventoryItems::Dye, 3.0), (InventoryItems::Wine, 3.0)],
    ],
    output: vec![(InventoryItems::Scrolls, 8.0)],
    time_requirement: 84.0,
    receipe_level: ReceipeLevel::Basic,
});
