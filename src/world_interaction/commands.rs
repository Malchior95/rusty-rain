use std::collections::LinkedList;

use log::info;

use crate::{
    config::buildings::Buildings,
    math::Pos,
    world::{
        World,
        inventory::Inventory,
        structures::{Building, BuildingBase, build_zone::BuildZone},
        world_map::TileType,
    },
};

pub enum BuildMethod {
    SpawnExisting,
    SpawnBuildZone,
}

pub fn build<'a>(
    world: &'a mut World,
    building: Buildings,
    pos: Pos,
    build_method: BuildMethod,
) -> Option<&'a mut Building> {
    let data = building.get_data();

    if !world.map.can_build(&pos, data.height, data.width) {
        info!("Cannot build {} at {}", building, pos);
        return None;
    }

    let building_behaviour = data.building_behaviour.to_default();

    let building_base = BuildingBase {
        pos,
        workers: LinkedList::new(),
        max_workers: data.max_workers,
        //TODO: what should be the output limit?
        output: Inventory::limited(10.0),
        building,
    };

    let final_building = Building {
        building_base,
        building_behaviour,
    };

    match build_method {
        BuildMethod::SpawnExisting => {
            world.shops.push_back(final_building);

            world
                .map
                .build(&pos, data.width, data.height, || TileType::Structure(building));

            return Some(world.shops.back_mut().unwrap());
        }
        BuildMethod::SpawnBuildZone => {
            let build_zone = BuildZone::new(final_building);
            world.build_zones.push_back(build_zone);

            world
                .map
                .build(&pos, data.width, data.height, || TileType::BuildZone(building));

            return Some(&mut world.build_zones.back_mut().unwrap().building);
        }
    }
}
