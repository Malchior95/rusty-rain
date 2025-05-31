use std::sync::LazyLock;

use crate::world::inventory::{InventoryItem, InventoryItems};

use super::ShopType;

pub struct BuildData {
    pub build_time: f32,
    pub materials_required: Vec<InventoryItems>,
    pub height: u8,
    pub width: u8,
}

impl ShopType {
    pub fn get_build_data(&self) -> &'static BuildData {
        match self {
            ShopType::MainHearth(_) => &HEARTH,
            ShopType::MainStore(_) => &MAIN_STORE,
            ShopType::Gatherer(shop) => {
                match &shop.data.resource_type {
                    crate::world::world_map::resources::ResourceType::Tree => &WOODCUTTER,
                    crate::world::world_map::resources::ResourceType::Berries => &HERBALIST,
                    crate::world::world_map::resources::ResourceType::Herbs => &HERBALIST, //TODO:
                }
            }
            ShopType::Producer(_) => {
                //TODO: differentiate producers
                &LUMBERMILL
            }
        }
    }
}

//wanted to use const here, but apparently Rust inlines those to make them constructed per each
//use. Here I wanted an immutable reference to a static memory bit.
//ok, what I said above is very shallow. Actually
//const:
//
//    Have no fixed address in memory
//    They’re inlined to each place which uses them, this means they are put directly into the binary on the places which use them.
//    Usually faster runtime but bigger executable file because it doesn't have to look up an address like static
//
//static:
//
//    Have a fixed address in memory
//    Their value is loaded from this fixed address each place which uses them.
//    Usually slower runtime because we need to perform the extra instruction of loading the data from the fixed address.
//    However this could result in a smaller executable file (only when it is used frequently) because
//    it doesn't have to have multiple copies of the value baked into the executable.
//
//The data below will not be used frequently in the code I think.

//OK, it turns out static and const fields are very limited and cannot, for example use Vecs.
//Shame.

//static HEARTH: LazyLock<BuildData> = LazyLock::new(|| BuildData {
//    build_time: 60.0,
//    materials_required: vec![], //hearth is free!
//    height: 4,
//    width: 4,
//});
//static MAIN_STORE: LazyLock<BuildData> = LazyLock::new(|| BuildData {
//    build_time: 60.0,
//    materials_required: vec![], //first store is free!
//    height: 3,
//    width: 4,
//});
//static WOODCUTTER: LazyLock<BuildData> = LazyLock::new(|| BuildData {
//    build_time: 60.0,
//    materials_required: Vec::from([(InventoryItem::Wood, 10.0)]),
//    height: 4,
//    width: 4,
//});
//static LUMBERMILL: LazyLock<BuildData> = LazyLock::new(|| BuildData {
//    build_time: 60.0,
//    materials_required: vec![(InventoryItem::Wood, 10.0)],
//    height: 4,
//    width: 4,
//});
//static HERBALIST: LazyLock<BuildData> = LazyLock::new(|| BuildData {
//    build_time: 60.0,
//    materials_required: vec![(InventoryItem::Wood, 10.0)],
//    height: 4,
//    width: 4,
//});

//The code below was generated

macro_rules! static_build_data {
    ($name:ident, [$($item:expr),*], $w:expr, $h:expr) => {
        static $name: LazyLock<BuildData> = LazyLock::new(|| BuildData {
            build_time: 60.0,
            materials_required: vec![$($item),*],
            width: $w,
            height: $h,
        });
    };
}

static_build_data!(HEARTH, [], 4, 4);
static_build_data!(MAIN_STORE, [], 4, 3);
static_build_data!(WOODCUTTER, [(InventoryItem::Wood, 10.0)], 2, 2);
static_build_data!(HERBALIST, [(InventoryItem::Wood, 10.0)], 2, 2);
static_build_data!(LUMBERMILL, [(InventoryItem::Wood, 10.0)], 3, 2);
//HEY! This is fricking nice!!
