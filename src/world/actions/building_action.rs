use log::{error, info};

use crate::{
    math::Pos,
    world::{
        World,
        structures::build_zone::BuildZone,
        world_map::{TileType, WorldMap},
    },
};

use super::{ActionResult, TransitAction, TransitActionResult};

pub struct BuildingAction {
    pub state: BuildingActionInternalState,
    pub build_zone: Option<BuildZone>,
}

pub enum BuildingActionInternalState {
    Going(TransitAction),
    Building,
}

pub enum BuildingActionResult {
    InProgress(Pos),
    Completed,
}

impl BuildingAction {
    pub fn new(
        path: Vec<Pos>,
        map: &WorldMap,
        build_zone: BuildZone,
    ) -> Self {
        Self {
            state: BuildingActionInternalState::Going(TransitAction::new(path, map)),
            build_zone: Some(build_zone),
        }
    }

    pub fn continue_action(
        &mut self,
        world: &mut World,
        delta: f32,
    ) -> BuildingActionResult {
        match &mut self.state {
            BuildingActionInternalState::Going(transit_action) => {
                let result = transit_action.continue_action(delta);

                match result {
                    TransitActionResult::InProgress(pos) => {
                        return BuildingActionResult::InProgress(pos);
                    }
                    TransitActionResult::Completed(pos) => {
                        self.state = BuildingActionInternalState::Building;

                        return BuildingActionResult::InProgress(pos);
                    }
                }
            }
            BuildingActionInternalState::Building => {
                return progres_build_action(world, &mut self.build_zone, delta);
            }
        }
    }
}

fn progres_build_action(
    world: &mut World,

    maybe_build_zone: &mut Option<BuildZone>,
    delta: f32,
) -> BuildingActionResult {
    let build_zone = if let Some(bz) = maybe_build_zone {
        bz
    } else {
        error!(
            "Building was already built, but the action was not properly completed by the invoking party. The action was invoked in the completed state."
        );
        return BuildingActionResult::Completed;
    };

    let action = build_zone.progress.continue_action(delta);

    if let ActionResult::InProgress = action {
        return BuildingActionResult::InProgress(build_zone.building.building_base.pos);
    }

    let data_taken = maybe_build_zone.take().unwrap(); //safe unwrap, I already checked if the data
    //is there

    let shop = data_taken.building;

    //TODO: this function assumes the check was already made and the build_zone footprint matches
    //the building's footprint - verify

    let config = shop.building_base.building.get_data();

    world
        .map
        .build(&shop.building_base.pos, config.width, config.height, || {
            TileType::Structure(shop.building_base.building)
        });

    world.shops.push_back(shop);

    info!("Build action completed successfully.");

    return BuildingActionResult::Completed;
}
