use log::info;

use crate::ai::a_star;

//I just had Rust implicilty copy this, without putting it back... so the actual value in enum was
//never updated. Need to be careful with Copy
#[derive(Debug, Clone)]
pub struct BasicAction {
    pub progress: f32,
    pub requirement: f32,
}

#[derive(Clone, Copy, Default)]
pub enum ActionResult {
    #[default]
    InProgress,
    Completed,
}

impl BasicAction {
    pub fn new(requirement: f32) -> Self {
        Self {
            progress: 0.0,
            requirement,
        }
    }

    pub fn process(&mut self, delta: f32) -> ActionResult {
        self.progress += delta;
        if self.progress > self.requirement {
            ActionResult::Completed
        } else {
            ActionResult::InProgress
        }
    }
}

#[derive(Clone)]
pub struct HaulAction {
    pub progress: f32,
    pub requirement: f32,
}

impl HaulAction {
    pub fn process(&mut self, delta: f32) -> ActionResult {
        self.progress += delta;
        if self.progress > self.requirement {
            ActionResult::Completed
        } else {
            ActionResult::InProgress
        }
    }
}

#[derive(Clone)]
pub struct ChopWood {
    pub progress: f32,
    pub requirement: f32,
    pub path: Vec<(usize, usize)>,
}

#[derive(Clone)]
pub struct Rest {
    pub progress: f32,
    pub requirement: f32,
    pub path: Vec<(usize, usize)>,
}
