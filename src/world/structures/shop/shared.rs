use log::{error, info};
use strum::IntoDiscriminant;

use crate::{
    ai::pathfinding::{self, pathfinding_helpers},
    math::Pos,
    world::{
        World,
        inventory::{Inventory, InventoryItem},
        receipes::Receipe,
        structures::ShopType,
        workers::Worker,
        world_map::{TileType, resources::ResourceType},
    },
};

pub fn handle_supply_complete(
    inventory: &mut Inventory,
    shop_inventory: &mut Inventory,
    shop_id: &String,
) {
    if inventory.is_empty() {
        return;
    }
    info!(
        "The following materials were added to {} inventory: {}",
        shop_id, inventory
    );
    shop_inventory.add_range(inventory);

    info!("{} has total items: {}", shop_id, shop_inventory);
}

pub fn handle_storing_complete(
    inventory: &mut Inventory,
    store_pos: Pos,
    world: &mut World,
    shop_id: &String,
) {
    if inventory.is_empty() {
        error!("Worker was bringing empty inventory to the store!");
        return;
    }

    //TODO: in the future I might want to store not only in the store, but in the
    //closest shop that requires those resources. Then I would not want to put to
    //output (as in the store), but to the input or data.inventory (as in Hearth)
    let maybe_store = world.shops.iter_mut().find(|s| s.location() == store_pos);

    let store = if let Some(store) = maybe_store {
        store
    } else {
        //store is not where it was before - return to building?

        error!("Worker was bringing items to the store, but the store is gone!");
        //FIXME:
        panic!()
    };
    info!("Adding to store action is being resolved by: {}", shop_id);
    info!("The following materials were added to Store's inventory: {}", inventory);

    store.inventory_mut().add_range(inventory);
    info!("The store now has: {}", store.inventory());
}

pub fn handle_poruction_complete(
    shop_input: &mut Inventory,
    shop_output: &mut Inventory,
    receipe: &Receipe,
) {
    shop_input.remove_range(&receipe.input);
    shop_output.add_range(&receipe.output);
    info!(
        "{} production complete. Store now has input: {} output: {}",
        receipe, shop_input, shop_output
    );
}

pub fn supply_command(
    worker: &mut Worker,
    shop_pos: Pos,
    world: &mut World,
    min_materials_to_consider_supplying: f32,
    materials_to_supply: InventoryItem,
    shop_id: &String,
) {
    if let Worker::Idle(idle_worker) = worker {
        let (closest_shop, path) = if let Some(x) = pathfinding_helpers::closest_shop_mut(shop_pos, world, |s| {
            s.inventory().get(&InventoryItem::Wood) >= min_materials_to_consider_supplying
        }) {
            x
        } else {
            info!("{} has no suitable stores with wood nearby.", shop_id);
            return; //remain idle
        };

        let stored_wood = closest_shop.inventory().get(&materials_to_supply);
        let to_take = f32::min(stored_wood, idle_worker.inventory.limit);

        closest_shop.inventory_mut().remove(materials_to_supply, to_take);
        let reservation = Inventory::from_iter([(materials_to_supply, to_take)]);

        info!(
            "{} will be supplying {} from {} at {}. Remaining in the store: {}.",
            idle_worker.name,
            reservation,
            closest_shop.discriminant(),
            path.last().unwrap(),
            closest_shop.inventory()
        );

        *worker = idle_worker.to_supplying(path, &world.map, reservation);
    }
}

pub fn store_command(
    worker: &mut Worker,
    world: &mut World,
    shop_pos: Pos,
    shop_output: &mut Inventory,
    storing_in_progress_flag: &mut bool,
    shop_id: &String,
) {
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
            return; //remain idle
        };

        *worker = idle_worker.to_storing(&world.map, path, shop_output);

        //once started storing - store everything
        if shop_output.total_items() <= 0.0 {
            *storing_in_progress_flag = false;
        } else {
            *storing_in_progress_flag = true;
        }
    }
}

pub fn gather_command(
    worker: &mut Worker,
    world: &mut World,
    resource_type: &ResourceType,
    shop_id: &String,
) {
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
            return; //remain idle
        };

        *worker = idle_worker.to_gathering(path, &mut world.map);
    }
}

pub fn produce_command(
    worker: &mut Worker,
    receipe: &Receipe,
    shop_id: &String,
) {
    if let Worker::Idle(idle_worker) = worker {
        info!("{} (worker {}) is producing {}.", shop_id, idle_worker.name, receipe);
        *worker = idle_worker.to_producing(receipe);
    }
}
