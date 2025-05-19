use std::fmt::Display;

use strum_macros::{Display, EnumIs};

use crate::math::Pos;

use super::{inventory::InventoryItem, structures::ShopTypeDiscriminants};

pub struct WorldMap {
    //TODO: in the future, I will definitelly want to have layers of the map - e.g. background with
    //resources, bogs, lakes, and empty, and foreground with structures, trees, etc. Imagine
    //building a structure on a resource and not delete it.
    //TODO: maybe also a layer for systems> e.g. pipes, wires... like ONI!
    pub map: Vec<Vec<TileType>>,
    //TODO: can I use an array?
    //pub map: [[TileType; A]; B]
}
#[derive(Default, EnumIs, Display)]
pub enum TileType {
    #[default]
    Empty,
    Road,
    Structure(ShopTypeDiscriminants),
    Tree,
    Resource(InventoryItem),
}

impl PartialEq for TileType {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        match (self, other) {
            (Self::Structure(l0), Self::Structure(r0)) => l0 == r0,
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
        let tiles = (0..height).map(|_| (0..width).map(|_| TileType::Empty).collect()).collect();
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
                    world.map[y][x] = TileType::Tree;
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
        if y + height as usize >= self.map.len() {
            return false;
        }

        if x + width as usize >= self.map.first().unwrap().len() {
            return false;
        }

        for h in 0..height {
            for w in 0..width {
                if !self.map[y + h as usize][x + w as usize].is_empty() {
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

    pub fn within_bounds(
        &self,
        pos: &Pos,
    ) -> bool {
        pos.x > 0 && pos.x < self.width() && pos.y > 0 && pos.y < self.height()
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
            TileType::Resource(_) => " ",
            TileType::Road => " ",
            TileType::Structure(shop_type) => match shop_type {
                ShopTypeDiscriminants::MainHearth => " ",
                ShopTypeDiscriminants::Woodcutter => "󰣈 ",
                //ShopType::Herbalist => "󰧻󱔐",
                ShopTypeDiscriminants::MainStore => "󰾁 ",
            },
            TileType::Tree => " ",
        }
    }

    pub fn cost(&self) -> f32 {
        match self {
            TileType::Empty => 1.0,
            TileType::Resource(_) => 2.0,
            TileType::Road => 0.7,
            TileType::Structure(_) => 10.0,
            TileType::Tree => 10.0,
        }
    }

    pub fn is_traversible(&self) -> bool {
        match self {
            TileType::Empty => true,
            TileType::Resource(_) => true,
            TileType::Road => true,
            TileType::Structure(_) => false,
            TileType::Tree => false,
        }
    }
}
