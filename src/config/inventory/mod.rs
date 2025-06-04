use crate::data_helpers::to_string::ToString;
use strum_macros::Display;

#[derive(Eq, PartialEq, Hash)]
pub struct InventoryItemConfig {
    pub name: String,
    pub is_fuel: bool,
    pub is_basic_food: bool,
    pub is_pack: bool,
}

impl ToString for (InventoryItems, f32) {
    fn to_string(&self) -> String {
        format!("{} {}", self.0, self.1)
    }
}

impl ToString for InventoryItems {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Display)]
pub enum InventoryItems {
    //basic materials
    Wood,
    Resin,

    Stone,
    Clay,
    PlantFiber,
    Reed,
    Leather,
    CopperOre,
    BoneMarrow,

    Grain,
    Herbs,

    //basic food
    Roots,
    Vegetables,
    Berries,
    Mushrooms,
    Eggs,
    Meat,
    Insects,

    //processed materials
    Plank,
    Bricks,
    Fabric,
    Pottery,
    Oil,
    Coal,
    Waterskins,
    CopperBars,
    CrystalizedDew,
    Barrels,
    Flour,

    //packs
    PackOfProvisions,
    PackOfCrops,
    PackOfBuildingMaterials,
    PackOfTradeGoods,
    PackOfLuxuryGoods,

    //processed food
    Skewers,
    Jerky,
    PickledGoods,
    Paste,
    Biscuits,
    Pie,
    Porridge,

    //trade items
    Coats,
    Boots,
    Scrolls,
    Dye,
    Incense,
    Wine,
    Ale,
    Tea,
    TrainingGear,

    //special
    Tools,
    Parts,
    WildfireEssence,
    Amber,
}
