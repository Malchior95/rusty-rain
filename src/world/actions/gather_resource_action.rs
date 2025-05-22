use log::info;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{
        inventory::Inventory,
        world_map::{TileType, WorldMap},
    },
};

use super::{ActionResult, BasicAction};

pub struct GatherResourcesAction {
    pub path_cost: Vec<f32>,
    pub gathering_cost: f32,
    pub path: Vec<Pos>,
    pub inventory: Inventory,
    internal_state: GatherResourceInternalAction,
}

enum GatherResourceInternalAction {
    Go(BasicAction),
    Gather(BasicAction),
    Return(BasicAction),
}

impl GatherResourcesAction {
    pub fn new<F>(
        from: Pos,
        map: &mut WorldMap,
        gathering_cost: f32,
        tile_type_check: F,
    ) -> Option<Self>
    where
        F: Fn(&TileType) -> bool,
    {
        info!("Worker wants to gather resources!");

        let path = pathfinding::dijkstra_closest_nontraversible(map, from, &tile_type_check)?;
        if path.is_empty() {
            return None;
        }
        let path_cost = map.path_to_cost(&path);
        let total_cost = path_cost.iter().sum::<f32>();

        let resource_location = path.last().unwrap();
        match map.get_mut(resource_location) {
            //TileType::Tree(_, being_cut) => *being_cut = true,
            TileType::Resource(_, _, being_cut) => *being_cut = true,
            _ => {}
        }
        Some(Self {
            path_cost,
            gathering_cost,
            path,
            inventory: Inventory::new(),
            internal_state: GatherResourceInternalAction::Go(BasicAction::new(total_cost)),
        })
    }

    pub fn process(
        &mut self,
        map: &mut WorldMap,
        inventory: &mut Inventory,
        delta: f32,
    ) -> ActionResult {
        if let GatherResourceInternalAction::Go(action) = &mut self.internal_state {
            let result = action.process(delta);
            if let ActionResult::Completed = result {
                let location = self.path.last().unwrap();

                info!("Worker started gathering resource at {}", location);
                self.internal_state = GatherResourceInternalAction::Gather(BasicAction::new(self.gathering_cost));
            }
            return ActionResult::InProgress;
        };

        if let GatherResourceInternalAction::Gather(action) = &mut self.internal_state {
            let result = action.process(delta);
            if let ActionResult::Completed = result {
                let resource_pos = self.path.last().unwrap(); //collection is not empty
                let resource_tile = map.get_mut(resource_pos);

                let total_path_cost = self.path_cost.iter().sum::<f32>();

                match resource_tile {
                    TileType::Resource(resource_type, resource_charge, being_cut) => {
                        let mut gathered_resources = resource_charge.gather();

                        for (key, amount) in gathered_resources.drain() {
                            self.inventory.add(key, amount);
                        }

                        *being_cut = false;
                        info!("Worker gathered resource {}", resource_type);
                        if resource_charge.current <= 0.0 {
                            *resource_tile = TileType::Empty;
                            info!("resource node depleted at {}!", resource_pos);
                        }
                    }
                    _ => {
                        info!("Did not find any resource to gather. Was the node depleted?");

                        self.internal_state = GatherResourceInternalAction::Return(BasicAction::new(total_path_cost));
                        return ActionResult::InProgress;
                    }
                }

                self.internal_state = GatherResourceInternalAction::Return(BasicAction::new(total_path_cost));
            }
            return ActionResult::InProgress;
        };

        if let GatherResourceInternalAction::Return(action) = &mut self.internal_state {
            let result = action.process(delta);
            if let ActionResult::Completed = result {
                for (key, amount) in &mut self.inventory.drain() {
                    inventory.add(key, amount);
                    let current_amount = inventory.get(&key);
                    info!("Worker returned to shop with {}. Current inventory: {}", &key, current_amount + amount);
                }
                return ActionResult::Completed;
            }
        };

        return ActionResult::InProgress;
    }
}
