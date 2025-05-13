use crate::ai::a_star;

#[derive(Clone, Default)]
pub enum Action {
    #[default]
    Idle,
    Rest(Rest),
    Haul(Haul),
    ChopWood(ChopWood),
}

impl Action {
    pub fn progress(self) -> Action {
        todo!()
    }
}

#[derive(Clone)]
pub struct Haul {
    pub progress: f32,
    pub requirement: f32,
    pub path: Vec<(usize, usize)>,
}

#[derive(Clone)]
pub struct ChopWood {
    pub progress: f32,
    pub requirement: f32,
    pub path: Vec<(usize, usize)>,
}

impl ChopWood {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            requirement: 10.0,
            path: todo!(), //FIXME:
        }
    }
}

#[derive(Clone)]
pub struct Rest {
    pub progress: f32,
    pub requirement: f32,
    pub path: Vec<(usize, usize)>,
}

impl Rest {
    pub fn new() -> Self {
        Self {
            progress: 0.0,
            requirement: 90.0,
            path: todo!(), //FIXME:
        }
    }
}
