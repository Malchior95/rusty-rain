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
        }
    }
}
