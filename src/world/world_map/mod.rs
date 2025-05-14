use std::{char, fmt::Display};

use crate::math::Pos;

#[derive(Clone)]
pub struct WorldMap {
    pub map: Vec<Vec<TileType>>,
}
#[derive(Clone, Copy, Default, PartialEq)]
pub enum TileType {
    #[default]
    Empty,
    Road,
    Structure, //TODO: maybe ref a structure
    Tree,      //TODO: maybe trees are different
               //TODO: more
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
    pub fn build(&mut self, x: usize, y: usize, width: u8, height: u8) {
        for h in 0..height {
            for w in 0..width {
                self.map[y + h as usize][x + w as usize] = TileType::Structure;
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
            crate::world::world_map::TileType::Empty => 1.0,
            crate::world::world_map::TileType::Road => 0.7,
            crate::world::world_map::TileType::Structure => 10.0,
            crate::world::world_map::TileType::Tree => 10.0,
        }
    }

    pub fn is_traversible(&self, pos: &Pos) -> bool {
        match self.map[pos.y][pos.x] {
            crate::world::world_map::TileType::Empty => true,
            crate::world::world_map::TileType::Road => true,
            crate::world::world_map::TileType::Structure => false,
            crate::world::world_map::TileType::Tree => false,
        }
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
            TileType::Road => " ",
            TileType::Structure => " ",
            TileType::Tree => " ",
        }
    }
}
