use log::info;
use strum::IntoDiscriminant;

use crate::{
    ai::pathfinding::{self, pathfinding_helpers},
    math::Pos,
    world::{
        World,
        inventory::{Inventory, InventoryItem, InventoryItems},
        receipes::Receipe,
        structures::ShopType,
        workers::Worker,
        world_map::{TileType, resources::ResourceType},
    },
};

pub fn handle_supply_complete(
    inventory: Vec<InventoryItems>,
    shop_inventory: &mut Inventory,
    shop_id: &String,
) {
    shop_inventory.add_range(inventory);

    info!("{} has total items: {}", shop_id, shop_inventory);
}

pub fn handle_poruction_complete(
    shop_input: &mut Inventory,
    shop_output: &mut Inventory,
    receipe: Receipe,
) {
    info!("{} production complete.", receipe);

    shop_input.remove_range(receipe.input);
    shop_output.add_range(receipe.output);

    info!("Store now has input: {} output: {}", shop_input, shop_output);
}

pub fn supply_command(
    worker: Worker,
    shop_pos: Pos,
    world: &mut World,
    min_materials_to_consider_supplying: f32,
    materials_to_supply: InventoryItem,
    shop_id: &String,
) -> Worker {
    //only idle worker can start supplying
    if let Worker::Idle(idle_worker) = worker {
        let (closest_shop, path) = if let Some(x) = pathfinding_helpers::closest_shop_mut(shop_pos, world, |s| {
            s.get_non_generic().output.get(&materials_to_supply) >= min_materials_to_consider_supplying
        }) {
            x
        } else {
            info!("{} has no suitable stores with wood nearby.", shop_id);
            return Worker::Idle(idle_worker); //remain idle
        };

        let stored_wood = closest_shop.get_non_generic().output.get(&materials_to_supply);
        let to_take = f32::min(stored_wood, idle_worker.inventory.limit);

        closest_shop
            .get_non_generic_mut()
            .output
            .remove(&materials_to_supply, to_take);
        let reservation = (materials_to_supply, to_take);

        info!(
            "{} will be supplying {} {} from {} at {}. Remaining in the store: {}.",
            idle_worker.name,
            reservation.0,
            reservation.1,
            closest_shop.discriminant(),
            path.last().unwrap(),
            closest_shop.get_non_generic().output
        );

        return idle_worker.to_supplying(path, &world.map, reservation);
    }
    return worker;
}

pub fn store_command(
    worker: Worker,
    world: &mut World,
    shop_pos: Pos,
    shop_output: &mut Inventory,
    storing_in_progress_flag: &mut bool,
    shop_id: &String,
) -> Worker {
    if let Worker::Idle(idle_worker) = worker {
        //store items
        info!("{} is storing resources. Current inventory: {}", shop_id, shop_output);

        let (_, path) = if let Some(x) = pathfinding_helpers::closest_shop_mut(shop_pos, world, |s| {
            //TODO: just bring to the store. In the future - maybe consider
            //bringing to the closest shop that needs materials?
            if let ShopType::MainStore(_) = s { true } else { false }
        }) {
            x
        } else {
            info!("{} has no suitable stores nearby.", shop_id);
            return Worker::Idle(idle_worker); //remain idle
        };

        //once started storing - store everything
        if shop_output.total_items() <= 0.0 {
            *storing_in_progress_flag = false;
        } else {
            *storing_in_progress_flag = true;
        }
        return idle_worker.to_storing(&world.map, path, shop_output);
    }
    return worker;
}

pub fn gather_command(
    worker: Worker,
    world: &mut World,
    resource_type: &ResourceType,
    shop_id: &String,
) -> Worker {
    if let Worker::Idle(idle_worker) = worker {
        let maybe_path = pathfinding::dijkstra_closest(&world.map, idle_worker.pos, |t| {
            if let TileType::Resource(r, _, being_cut) = t {
                if r == resource_type && !being_cut { true } else { false }
            } else {
                false
            }
        });

        let path = if let Some(path) = maybe_path {
            path
        } else {
            info!("{} has no suitable resource nodes nearby.", shop_id);
            return Worker::Idle(idle_worker); //remain idle
        };

        return idle_worker.to_gathering(path, &mut world.map);
    }
    return worker;
}

pub fn produce_command(
    worker: Worker,
    receipe: &Receipe,
    shop_id: &String,
) -> Worker {
    if let Worker::Idle(idle_worker) = worker {
        info!("{} (worker {}) is producing {}.", shop_id, idle_worker.name, receipe);
        return idle_worker.to_producing(receipe);
    }
    return worker;
}
