use std::fmt::Display;

use crate::{math::Pos, world::world_map::WorldMap};

pub struct PathDrawer<'a> {
    pub map: &'a WorldMap,
    pub path: &'a Vec<Pos>,
}
impl<'a> PathDrawer<'a> {
    fn get_path_marker(&self, pos: &Pos) -> &str {
        let current_index = self.path.iter().position(|x| x == pos);
        const DEFAULT_MARKER: &str = "\u{f29f} ";
        match current_index {
            Some(index) => {
                let next = self.path.get(index + 1);
                if let Some(next) = next {
                    //not that lower values are at the top of the screen when drawin in terminal
                    if next.y < pos.y {
                        return " ";
                    }
                    if next.y > pos.y {
                        return " ";
                    }
                    if next.x < pos.x {
                        return " ";
                    }
                    if next.x > pos.x {
                        return " ";
                    }
                    DEFAULT_MARKER
                } else {
                    DEFAULT_MARKER
                }
            }
            None => DEFAULT_MARKER,
        }
    }
    fn number_steps(&self, pos: &Pos) -> String {
        let maybe_index = self.path.iter().position(|x| x == pos);
        if let Some(index) = maybe_index {
            return format!("{:2}", index);
        }
        "..".to_string()
    }
}
impl<'a> Display for PathDrawer<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.map.width() < 100 {
            if self.map.height() < 100 {
                write!(f, "  ")?;
            }
            for x in 0..self.map.width() {
                write!(f, "{:2}", x)?;
            }
            writeln!(f)?;
        }
        for (iy, y) in self.map.map.iter().enumerate() {
            if self.map.height() < 100 {
                write!(f, "{:2}", iy)?;
            }
            for (ix, x) in y.iter().enumerate() {
                let pos = Pos { x: ix, y: iy };
                if self.path.contains(&pos) {
                    let marker = self.get_path_marker(&pos);
                    write!(f, "{}", marker)?;
                    //let numbering = self.number_steps(&pos);
                    //write!(f, "{}", numbering)?;
                } else {
                    write!(f, "{}", x.to_char())?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
