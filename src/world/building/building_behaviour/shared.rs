use log::info;
use strum::IntoDiscriminant;

use crate::{
    ai::pathfinding::pathfinding_helpers,
    config::inventory::InventoryItems,
    data_helpers::to_string::ToString,
    math::Pos,
    world::{World, inventory::Inventory, worker::Worker},
};

pub fn supply_command(
    worker: Worker,
    shop_pos: Pos,
    world: &mut World,
    materials_to_supply_any_of: &Vec<InventoryItems>,
    shop_id: &String,
) -> Worker {
    //only idle worker can start supplying
    let idle_worker = if let Worker::Idle(idle_worker) = worker {
        idle_worker
    } else {
        return worker;
    };

    let (closest_shop, path) = if let Some(x) = pathfinding_helpers::closest_shop_mut(shop_pos, world, |s| {
        s.building_base.output.has_any_of(materials_to_supply_any_of)
    }) {
        x
    } else {
        info!(
            "{} has no suitable stores with any of {} nearby...",
            shop_id,
            materials_to_supply_any_of.to_string()
        );
        return Worker::Idle(idle_worker); //remain idle
    };

    let reservation = pick_one_of(
        &closest_shop.building_base.output,
        materials_to_supply_any_of,
        idle_worker.inventory.limit,
    )
    .unwrap(); //can safely unwrap - just checked if the store has the materials

    closest_shop.building_base.output.remove(&reservation.0, reservation.1);

    info!(
        "{} will be supplying {} {} from {} at {}. Remaining in the store: {}.",
        idle_worker.name,
        reservation.0,
        reservation.1,
        closest_shop.building_behaviour.discriminant(),
        path.last().unwrap(),
        closest_shop.building_base.output
    );

    return idle_worker.to_supplying(path, &world.map, reservation);
}

///This method will prioritize the item of the highest quantity from the store
fn pick_one_of(
    inventory: &Inventory,
    materials_to_take_variant: &Vec<InventoryItems>,
    limit: f32,
) -> Option<(InventoryItems, f32)> {
    if materials_to_take_variant.is_empty() || inventory.is_empty() {
        return None;
    }

    let mut union: Vec<_> = inventory
        .iter()
        .filter(|(key, _)| materials_to_take_variant.iter().any(|k| *key == k))
        .collect();

    union.sort_by(|l, r| l.1.total_cmp(r.1));

    let best_material = if let Some(a) = union.first() {
        a.0.clone() //clone to be returned from the func
    } else {
        return None;
    };

    let best = inventory.get(&best_material);

    let to_take = f32::min(best, limit);

    Some((best_material, to_take))
}

pub fn store_command(
    worker: Worker,
    world: &mut World,
    shop_output: &mut Inventory,
    shop_id: &String,
) -> Worker {
    let idle_worker = if let Worker::Idle(idle_worker) = worker {
        idle_worker
    } else {
        return worker;
    };

    //store items
    info!("{} is storing resources. Current inventory: {}", shop_id, shop_output);

    let mut storing_or_idle_worker = idle_worker.try_storing(world);
    if let Worker::Storing(sw) = &mut storing_or_idle_worker {
        info!("{} is storing materials, current pos {}.", sw.name, sw.pos);

        transfer_until_full(shop_output, &mut sw.inventory);

        info!("{} now has the follwoing materials {}", sw.name, sw.inventory);
        info!("The follwoing materials remain in the shop {}", shop_output);
    }

    return storing_or_idle_worker;
}

fn transfer_until_full(
    source: &mut Inventory,
    target: &mut Inventory,
) {
    if target.limit <= 0.0 || source.total_items() < target.limit - target.total_items() {
        for (key, items) in source.drain() {
            target.add(&key, items);
        }
    } else {
        for (&key, items) in source.inv.iter_mut() {
            let remaining_capacity = target.limit - target.total_items();
            if remaining_capacity <= 0.0 {
                return;
            }
            let to_transfer = f32::min(remaining_capacity, *items);
            *items -= to_transfer;
            target.add(&key, to_transfer);
        }
    }
}
