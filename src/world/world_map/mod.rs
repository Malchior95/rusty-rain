use std::{char, fmt::Display};

use crate::math::Pos;

use super::structures::shop::ShopType;

#[derive(Clone)]
pub struct WorldMap {
    pub map: Vec<Vec<TileType>>,
}
#[derive(Clone, Default)]
pub enum TileType {
    #[default]
    Empty,
    MainHearth,
    Road,
    Structure(ShopType),
    Tree,
    Resource,
}

impl TileType {
    pub fn is_match(&self, other: &Self, exact: bool) -> bool {
        match (self, other) {
            (Self::Structure(l0), Self::Structure(r0)) => !exact || (l0 == r0),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl PartialEq for TileType {
    fn eq(&self, other: &Self) -> bool {
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

    pub fn new(width: usize, height: usize) -> WorldMap {
        let tiles = vec![vec![TileType::Empty; width]; height];
        WorldMap { map: tiles }
    }

    pub fn new_test(width: usize, height: usize) -> WorldMap {
        let mut world = WorldMap::new(width, height);

        //create boundary of trees
        for y in 0..height {
            for x in 0..width {
                if [0, 1, height - 2, height - 1].contains(&y)
                    || [0, 1, width - 2, width - 1].contains(&x)
                {
                    world.map[y][x] = TileType::Tree;
                }
            }
        }

        world
    }

    /// sets the rectangular region as containing a structure. You MUST call can_build first, or
    /// you are risking a panic or invalid game state
    pub fn build(&mut self, x: usize, y: usize, width: u8, height: u8, tile_type: TileType) {
        for h in 0..height {
            for w in 0..width {
                self.map[y + h as usize][x + w as usize] = tile_type.clone();
            }
        }
    }

    pub fn can_build(&self, x: usize, y: usize, width: u8, height: u8) -> bool {
        if y + height as usize >= self.map.len() {
            return false;
        }

        if x + width as usize >= self.map.first().unwrap().len() {
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

    pub fn get_cost(&self, pos: &Pos) -> f32 {
        match self.map[pos.y][pos.x] {
            TileType::Empty => 1.0,
            TileType::Resource => 2.0,
            TileType::MainHearth => 10.0,
            TileType::Road => 0.7,
            TileType::Structure(_) => 10.0,
            TileType::Tree => 10.0,
        }
    }

    pub fn is_traversible(&self, pos: &Pos) -> bool {
        match self.map[pos.y][pos.x] {
            TileType::Empty => true,
            TileType::Resource => true,
            TileType::MainHearth => false,
            TileType::Road => true,
            TileType::Structure(_) => false,
            TileType::Tree => false,
        }
    }

    pub fn get(&self, pos: &Pos) -> &TileType {
        &self.map[pos.y][pos.x]
    }

    pub fn within_bounds(&self, pos: &Pos) -> bool {
        pos.x > 0 && pos.x < self.width() && pos.y > 0 && pos.y < self.height()
    }

    pub fn is_match(&self, pos: &Pos, tile_type: &TileType, exact: bool) -> bool {
        self.map[pos.y][pos.x].is_match(tile_type, exact)
    }
}

impl Display for WorldMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
            TileType::Resource => " ",
            TileType::MainHearth => " ",
            TileType::Road => " ",
            TileType::Structure(shop_type) => match shop_type {
                ShopType::Woodcutter => "󰣈 ",
                ShopType::Herbalist => "󰧻󱔐",
                ShopType::Store => "󰾁 ",
            },
            TileType::Tree => " ",
        }
    }
}
