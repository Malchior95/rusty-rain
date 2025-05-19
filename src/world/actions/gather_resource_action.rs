use std::collections::HashMap;

use log::info;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{
        inventory::InventoryItem,
        world_map::{TileType, WorldMap},
    },
};

use super::{ActionResult, BasicAction};

pub struct GatherResourcesAction {
    pub path_cost: Vec<f32>,
    pub gathering_cost: f32,
    pub path: Vec<Pos>,
    pub inventory: HashMap<InventoryItem, f32>,
    pub tile_type: TileType,
    internal_state: GatherResourceInternalAction,
}

pub enum GatherResourceInternalAction {
    Go(BasicAction),
    Gather(BasicAction),
    Return(BasicAction),
}

impl GatherResourcesAction {
    pub fn new(
        from: Pos,
        map: &WorldMap,
        gathering_cost: f32,
        tile_type: TileType,
    ) -> Option<Self> {
        info!("Worker wants to gather resources!");

        let path = pathfinding::breadth_first_closest_nontraversible(map, from, &tile_type)?;
        let path_cost = map.path_to_cost(&path);
        let total_cost = path_cost.iter().sum::<f32>();

        Some(Self {
            path_cost,
            gathering_cost,
            path,
            inventory: HashMap::new(),
            internal_state: GatherResourceInternalAction::Go(BasicAction::new(total_cost)),
            tile_type,
        })
    }

    pub fn process(
        &mut self,
        inventory: &mut HashMap<InventoryItem, f32>,
        delta: f32,
    ) -> ActionResult {
        if let GatherResourceInternalAction::Go(action) = &mut self.internal_state {
            let result = action.process(delta);
            if let ActionResult::Completed = result {
                info!("Worker started gathering {}", self.tile_type);
                self.internal_state = GatherResourceInternalAction::Gather(BasicAction::new(self.gathering_cost));
            }
            return ActionResult::InProgress;
        };

        if let GatherResourceInternalAction::Gather(action) = &mut self.internal_state {
            let result = action.process(delta);
            if let ActionResult::Completed = result {
                //FIXME: deplete resource by one... or whatever
                self.inventory.insert(InventoryItem::Wood, 5.0);
                info!("Worker gathered resource");

                let total_cost = self.path_cost.iter().sum::<f32>();
                self.internal_state = GatherResourceInternalAction::Return(BasicAction::new(total_cost));
            }
            return ActionResult::InProgress;
        };

        if let GatherResourceInternalAction::Return(action) = &mut self.internal_state {
            let result = action.process(delta);
            if let ActionResult::Completed = result {
                for (key, amount) in &mut self.inventory.drain() {
                    let current_amount = inventory.get(&key).unwrap_or(&0.0);
                    inventory.insert(key, *current_amount + amount);

                    let current_amount = inventory.get(&key).unwrap_or(&0.0);

                    info!("Workered returned to shop with resources. Current inventory: {}", *current_amount);
                }
                return ActionResult::Completed;
            }
        };

        return ActionResult::InProgress;
    }
}
