use crate::math::Pos;

use super::world_map::WorldMap;

pub mod gathering_action;
pub mod taking_break_action;
#[derive(Clone)]
pub struct BasicAction {
    pub progress: f32,
    pub requirement: f32,
}

#[derive(Default)]
pub enum ActionResult {
    #[default]
    InProgress,
    Completed,
}

pub struct TransitAction {
    pub path: Vec<Pos>,
    pub path_cost: Vec<f32>,
    pub progress: f32,
    pub requirement: f32,
}

pub enum TransitActionResult {
    InProgress(Pos),
    Completed(Pos),
}

impl TransitAction {
    pub fn new(
        path: Vec<Pos>,
        map: &WorldMap,
    ) -> Self {
        let path_cost: Vec<f32> = path.iter().map(|p| map.get(p).cost()).collect();
        let requirement = path_cost.iter().sum::<f32>();
        Self {
            path,
            path_cost,
            progress: 0.0,
            requirement,
        }
    }

    pub fn continue_action(
        &mut self,
        delta: f32,
    ) -> TransitActionResult {
        self.progress += delta;

        if self.progress >= self.requirement {
            return TransitActionResult::Completed(self.path.last().unwrap().clone());
        }
        TransitActionResult::InProgress(self.current_pos())
    }

    fn current_pos(&self) -> Pos {
        let mut acc = 0.0;
        for (i, cost) in self.path_cost.iter().enumerate() {
            acc += cost;
            if acc >= self.progress {
                return self.path.get(i).unwrap_or(self.path.last().unwrap()).clone();
            }
        }
        self.path.last().unwrap().clone()
    }
}

impl BasicAction {
    pub fn new(requirement: f32) -> Self {
        Self {
            progress: 0.0,
            requirement,
        }
    }

    pub fn continue_action(
        &mut self,
        delta: f32,
    ) -> ActionResult {
        self.progress += delta;
        if self.progress > self.requirement {
            ActionResult::Completed
        } else {
            ActionResult::InProgress
        }
    }

    pub fn is_completed(&self) -> bool {
        self.progress > self.requirement
    }
}
