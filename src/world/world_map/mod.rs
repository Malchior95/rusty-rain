use std::fmt::Display;

use resources::{ResourceCharge, ResourceType};
use strum_macros::{Display, EnumDiscriminants, EnumIs};

use crate::math::Pos;

use super::structures::ShopTypeDiscriminants;
pub mod resources;

pub struct WorldMap {
    //TODO: in the future, I will definitelly want to have layers of the map - e.g. background with
    //resources, bogs, lakes, and empty, and foreground with structures, trees, etc. Imagine
    //building a structure on a resource and not delete it.
    //TODO: maybe also a layer for systems> e.g. pipes, wires... like ONI!
    pub map: Vec<Vec<TileType>>,
    //TODO: can I use an array?
    //If I want to, TileType needs to be Copy
    //pub map: [[TileType; A]; B]
}
#[derive(Default, Display, EnumDiscriminants)]
pub enum TileType {
    #[default]
    Empty,
    Road,
    Structure(ShopTypeDiscriminants),
    Resource(ResourceType, ResourceCharge, bool),
    //Tree(ResourceCharge, bool),
}

impl PartialEq for TileType {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Self::Structure(l0), Self::Structure(r0)) => l0 == r0,
            (Self::Resource(l0, _, _), Self::Resource(r0, _, _)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl WorldMap {
    pub fn height(&self) -> usize {
        self.map.len()
    }

    pub fn width(&self) -> usize {
        self.map.first().unwrap().len()
    }

    pub fn new(
        width: usize,
        height: usize,
    ) -> WorldMap {
        let tiles = (0..height)
            .map(|_| (0..width).map(|_| TileType::Empty).collect())
            .collect();
        WorldMap { map: tiles }
    }

    pub fn new_test(
        width: usize,
        height: usize,
    ) -> WorldMap {
        let mut world = WorldMap::new(width, height);

        //create boundary of trees
        for y in 0..height {
            for x in 0..width {
                if [0, 1, height - 2, height - 1].contains(&y) || [0, 1, width - 2, width - 1].contains(&x) {
                    world.map[y][x] = ResourceType::tile_tree();
                }
            }
        }

        world
    }

    /// sets the rectangular region as containing a structure. You MUST call can_build first, or
    /// you are risking a panic or invalid game state
    pub fn build<F>(
        &mut self,
        x: usize,
        y: usize,
        width: u8,
        height: u8,
        mut tile_type_factory: F,
    ) where
        F: FnMut() -> TileType,
    {
        //TODO: cloning occurs here. Can I disable cloning and create new items? How to do?
        for h in 0..height {
            for w in 0..width {
                self.map[y + h as usize][x + w as usize] = tile_type_factory();
            }
        }
    }

    pub fn can_build(
        &self,
        x: usize,
        y: usize,
        width: u8,
        height: u8,
    ) -> bool {
        if y + height as usize >= self.height() {
            return false;
        }

        if x + width as usize >= self.width() {
            return false;
        }

        for h in 0..height {
            for w in 0..width {
                if self.map[y + h as usize][x + w as usize] != TileType::Empty {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn get(
        &self,
        pos: &Pos,
    ) -> &TileType {
        &self.map[pos.y][pos.x]
    }

    pub fn get_mut(
        &mut self,
        pos: &Pos,
    ) -> &mut TileType {
        &mut self.map[pos.y][pos.x]
    }

    pub fn within_bounds(
        &self,
        pos: &Pos,
    ) -> bool {
        pos.x < self.width() && pos.y < self.height()
    }

    pub fn path_to_cost(
        &self,
        path: &Vec<Pos>,
    ) -> Vec<f32> {
        path.iter().map(|p| self.get(p).cost()).collect()
    }
}

impl Display for WorldMap {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.width() < 100 {
            if self.height() < 100 {
                write!(f, "  ")?;
            }
            for x in 0..self.width() {
                write!(f, "{:2}", x)?;
            }
            writeln!(f)?;
        }
        for (i, y) in self.map.iter().enumerate() {
            if self.height() < 100 {
                write!(f, "{:2}", i)?;
            }
            for x in y {
                write!(f, "{}", x.to_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TileType {
    pub fn to_char(&self) -> &str {
        match self {
            TileType::Empty => "  ",
            TileType::Resource(resource_type, _, _) => match resource_type {
                ResourceType::Tree => " ",
                ResourceType::Berries => " ",
                ResourceType::Herbs => " ",
                //_ => " ",
            },
            TileType::Road => " ",
            TileType::Structure(shop_type) => match shop_type {
                ShopTypeDiscriminants::MainHearth => " ",
                //ShopTypeDiscriminants::Woodcutter => "󰣈 ",
                ShopTypeDiscriminants::Gatherer => "󰧻󱔐",
                ShopTypeDiscriminants::MainStore => "󰾁 ",
            },
            //TileType::Tree(_, _) => " ",
        }
    }

    pub fn cost(&self) -> f32 {
        match self {
            TileType::Empty => 1.0,
            TileType::Resource(_, _, _) => 2.0,
            TileType::Road => 0.7,
            TileType::Structure(_) => 1.0, //this is the cost of entering the building...
                                           //TileType::Tree(_, _) => 10.0,
        }
    }

    pub fn is_traversible(&self) -> bool {
        match self {
            TileType::Empty => true,
            TileType::Resource(rt, _, _) => match rt {
                ResourceType::Tree => false,
                _ => true,
            },
            TileType::Road => true,
            TileType::Structure(_) => false,
            //TileType::Tree(_, _) => false,
        }
    }

    pub fn is_store(&self) -> bool {
        if let TileType::Structure(structure) = self {
            if let ShopTypeDiscriminants::MainStore = structure {
                return true;
            }
        }
        false
    }

    pub fn is_hearth(&self) -> bool {
        if let TileType::Structure(structure) = self {
            if let ShopTypeDiscriminants::MainHearth = structure {
                return true;
            }
        }
        false
    }
}
