use crate::{
    math::Pos,
    world::{
        inventory::InventoryItems,
        world_map::{TileType, WorldMap},
    },
};

use super::{ActionResult, BasicAction, TransitAction, TransitActionResult};

pub struct GatheringAction {
    pub state: GatheringActionInternalState,
    pub pos: Pos,
}

pub enum GatheringActionInternalState {
    Going(TransitAction),
    Gathering(BasicAction),
}

pub enum GatheringActionResult {
    InProgress(Pos),
    Completed(Vec<InventoryItems>),
}

impl GatheringAction {
    pub fn new(
        path: Vec<Pos>,
        map: &mut WorldMap,
    ) -> Self {
        let pos = path.first().unwrap().clone();
        let final_pos = path.last().unwrap();

        let resource = map.get_mut(final_pos);

        if let TileType::Resource(_, _, being_cut) = resource {
            *being_cut = true;
        }

        Self {
            state: GatheringActionInternalState::Going(TransitAction::new(path, map)),
            pos,
        }
    }

    pub fn continue_action(
        &mut self,
        map: &mut WorldMap,
        delta: f32,
    ) -> GatheringActionResult {
        match &mut self.state {
            GatheringActionInternalState::Going(transit_action) => {
                let result = transit_action.continue_action(delta);

                match result {
                    TransitActionResult::InProgress(pos) => self.pos = pos,
                    TransitActionResult::Completed(pos) => {
                        self.state = GatheringActionInternalState::Gathering(BasicAction::new(10.0));
                        self.pos = pos;

                        //arrived at the destination - check if resource still there :P

                        let resource = map.get(&self.pos);
                        if let TileType::Resource(_, _, _) = resource {
                        } else {
                            return GatheringActionResult::Completed(vec![]);
                        }
                    }
                }
            }
            GatheringActionInternalState::Gathering(basic_action) => {
                let result = basic_action.continue_action(delta);
                if let ActionResult::Completed = result {
                    let resource = map.get_mut(&self.pos);

                    if let TileType::Resource(_, charge, being_cut) = resource {
                        let inv = charge.gather();
                        *being_cut = false;

                        if charge.current <= 0.0 {
                            *resource = TileType::Empty;
                        }

                        return GatheringActionResult::Completed(inv);
                    }
                }
            }
        }

        GatheringActionResult::InProgress(self.pos)
    }
}
