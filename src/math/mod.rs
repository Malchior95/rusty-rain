use std::fmt::Display;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn new(
        x: usize,
        y: usize,
    ) -> Self {
        Self { x, y }
    }
}

impl Display for Pos {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
