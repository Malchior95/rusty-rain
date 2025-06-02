use strum_macros::Display;

#[derive(Eq, PartialEq, Hash)]
pub struct InventoryItem {
    pub name: String,
    pub is_fuel: bool,
    pub is_basic_food: bool,
    pub is_pack: bool,
}

#[derive(Hash, PartialEq, Eq, Copy, Clone, Display)]
pub enum BuiltInInventoryItems {
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
