use strum_macros::Display;

use super::inventory::InventoryItems;
use crate::world::building::BuildingBehaviourDiscriminants;

use super::receipes::Receipe;
pub mod building_configs;

pub struct BuildingConfig {
    pub name: &'static str,
    pub max_workers: u8,
    pub build_time: f32,
    pub build_materials: Vec<(InventoryItems, f32)>,
    pub production_receipes: Vec<&'static Receipe>,
    pub gathered_resource_types: Vec<InventoryItems>,
    pub width: u8,
    pub height: u8,
    pub building_behaviour: BuildingBehaviourDiscriminants,
}

#[derive(PartialEq, Eq, Clone, Copy, Display)]
pub enum Buildings {
    //Gatherer
    Woodcutter,
    Herbalist,
    Stonecutter,
    Harvester,
    Forager,
    Trapper,

    //essential
    MainHearth,
    MainStore,
    Hearth,
    Store,

    //Producer - materials
    CrudeWorkstation,
    MakeshiftPost,

    Workshop,
    Cooperage,
    Carpenter,
    Cobbler,
    Weaver,
    CLothier,
    Leatherworker,
    BrickOven,
    Brickyard,
    Press,
    Provisioner,
    Smelter,
    Smithy,
    Toolshop,
    Artisan,
    AlchemistHut,
    Teahouse,
    Brewery,
    Distillery,
    Tinctury,
    DruidsHut,
    Tinkerer,
    Manufactory,
    Supplier,
    Scribe,

    Lumbermill,
    Kiln,

    //Producer - food
    FieldKitchen,
    Butcher,
    Bakery,
    Cookhouse,
    Smokehouse,
    Beanery,
    Cellar,
    Cannery,
    Granary,
    Grill,
    Rainmill,
    Pantry,
    Apothecary,
    Furnace,
    Ranch,
    //Special
    // BlightPost,
    // GayserPump,
    // RainCollector,
    // TradingPost

    // +farms
    // +fertile soil buildings
    // +mine
    // +housing

    // Tavern,
    // Temple,
    // Monastery,
    // ClanHall,
    // Forum,
    // GuildHouse,
    // Market,
    // TeaDoctor,
    // BathHouse,
    // FeastHall,
    // ExplorersLodge,
}

impl Buildings {
    pub fn get_data(&self) -> &'static BuildingConfig {
        match self {
            Buildings::Woodcutter => &building_configs::WOODCUTTER,
            Buildings::Herbalist => todo!(),
            Buildings::Stonecutter => todo!(),
            Buildings::Harvester => todo!(),
            Buildings::Forager => todo!(),
            Buildings::Trapper => todo!(),
            Buildings::MainHearth => &building_configs::MAIN_HEARTH,
            Buildings::MainStore => &building_configs::MAIN_STORE,
            Buildings::CrudeWorkstation => todo!(),
            Buildings::MakeshiftPost => todo!(),
            Buildings::Lumbermill => &building_configs::LUMBERMILL,
            Buildings::Kiln => todo!(),
            Buildings::Brickyard => todo!(),
            Buildings::FieldKitchen => todo!(),
            Buildings::Butcher => todo!(),
            Buildings::Hearth => todo!(),
            Buildings::Store => todo!(),
            Buildings::Workshop => todo!(),
            Buildings::Cooperage => todo!(),
            Buildings::Carpenter => todo!(),
            Buildings::Cobbler => todo!(),
            Buildings::Weaver => todo!(),
            Buildings::CLothier => todo!(),
            Buildings::Leatherworker => todo!(),
            Buildings::BrickOven => todo!(),
            Buildings::Press => todo!(),
            Buildings::Provisioner => todo!(),
            Buildings::Smelter => todo!(),
            Buildings::Smithy => todo!(),
            Buildings::Toolshop => todo!(),
            Buildings::Artisan => todo!(),
            Buildings::AlchemistHut => todo!(),
            Buildings::Teahouse => todo!(),
            Buildings::Brewery => todo!(),
            Buildings::Distillery => todo!(),
            Buildings::Tinctury => todo!(),
            Buildings::DruidsHut => todo!(),
            Buildings::Tinkerer => todo!(),
            Buildings::Manufactory => todo!(),
            Buildings::Supplier => todo!(),
            Buildings::Scribe => todo!(),
            Buildings::Bakery => todo!(),
            Buildings::Cookhouse => todo!(),
            Buildings::Smokehouse => todo!(),
            Buildings::Beanery => todo!(),
            Buildings::Cellar => todo!(),
            Buildings::Cannery => todo!(),
            Buildings::Granary => todo!(),
            Buildings::Grill => todo!(),
            Buildings::Rainmill => todo!(),
            Buildings::Pantry => todo!(),
            Buildings::Apothecary => todo!(),
            Buildings::Furnace => todo!(),
            Buildings::Ranch => todo!(),
        }
    }
}
