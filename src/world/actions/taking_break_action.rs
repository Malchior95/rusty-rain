use crate::{math::Pos, world::world_map::WorldMap};

use super::{ActionResult, BasicAction, TransitAction, TransitActionResult};

pub struct TakingBreakAction {
    pub state: TakingBreakActionInternalState,
    pub pos: Pos,
}

pub enum TakingBreakActionInternalState {
    Going(TransitAction),
    TakingBreak(BasicAction),
}

pub enum TakingBreakActionResult {
    InProgress(Pos),
    Completed,
}

impl TakingBreakAction {
    pub const BREAK_TIME: f32 = 30.0;

    pub fn new(
        path: Vec<Pos>,
        map: &WorldMap,
    ) -> Self {
        let pos = path.first().unwrap().clone();

        Self {
            state: TakingBreakActionInternalState::Going(TransitAction::new(path, map)),
            pos,
        }
    }

    pub fn continue_action(
        &mut self,
        delta: f32,
    ) -> TakingBreakActionResult {
        match &mut self.state {
            TakingBreakActionInternalState::Going(transit_action) => {
                let result = transit_action.continue_action(delta);

                match result {
                    TransitActionResult::InProgress(pos) => self.pos = pos,
                    TransitActionResult::Completed(pos) => {
                        self.state = TakingBreakActionInternalState::TakingBreak(BasicAction::new(
                            TakingBreakAction::BREAK_TIME,
                        ));
                        self.pos = pos;
                    }
                }
            }
            TakingBreakActionInternalState::TakingBreak(basic_action) => {
                let result = basic_action.continue_action(delta);
                if let ActionResult::Completed = result {
                    return TakingBreakActionResult::Completed;
                }
            }
        }

        TakingBreakActionResult::InProgress(self.pos)
    }
}
