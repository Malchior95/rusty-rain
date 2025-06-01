use crate::ai::pathfinding::{self, pathfinding_helpers};
use crate::math::Pos;
use crate::world::inventory::{Inventory, InventoryItem, InventoryItems};
use crate::world::{
    World,
    structures::ShopTypeDiscriminants,
    workers::{Worker, worker::WorkerActionResult},
};

use super::Idle;
use super::worker_with_action::WorkerWithAction;

impl Worker {
    pub fn process_unassigned_worker(
        self,
        assigned_hearth_pos: Pos,
        world: &mut World,
        delta: f32,
    ) -> Worker {
        let (worker, result) =
            self.continue_action(assigned_hearth_pos, ShopTypeDiscriminants::MainHearth, delta, world);

        match result {
            WorkerActionResult::InProgress => {
                //action in progress
            }
            WorkerActionResult::BroughtToShop(items) => {
                assert!(items.is_empty());
                //INFO: Unassigned workers will never bring items to shop, altough they will
                //"return", therefore invoking this result
            }
            WorkerActionResult::ProductionComplete(_) => unreachable!("Unassigned workers will never produce."),
            WorkerActionResult::Idle => {
                if let Worker::Idle(idle_worker) = worker {
                    return schedule_new_work(idle_worker, world);
                }
            }
        }

        worker
    }
}

fn schedule_new_work(
    mut worker: WorkerWithAction<Idle>,
    world: &mut World,
) -> Worker {
    //If for whatever reason, no action can be performed on the first build_zone (e.g.
    //unreachable), I do not want it to just sit on front() and block everything. That's why I'm
    //popping it and shoving to the back. The next worker (or the same, if there is only 1), will
    //have a chance to handle the next build_zone on this or next tick.

    let build_zone = if let Some(bz) = world.build_zones.pop_front() {
        bz
    } else {
        return Worker::Idle(worker); //no build zones
    };

    if build_zone.is_delivery_complete() {
        let maybe_path = pathfinding::a_star(
            &world.map,
            worker.pos,
            build_zone.shop_type.get_non_generic().structure.pos,
        );
        if let Some(path) = maybe_path {
            return worker.to_building(&world.map, path, build_zone);
        }
    }

    let required_materials = build_zone.materials_required;

    //if no delivery completed, or unreachable, try supplying
    //TODO: consider taking materials from other buildings too
    //order by the amount of materials first
    let (closest_store, path_to_store) = if let Some(csp) =
        pathfinding_helpers::closest_shop_mut_2(worker.pos, &world.map, &mut world.shops, |s| {
            s.is_main_store() //for now only consider stores - in the future maybe also take from other places
            && has_any_of(s.get_non_generic().output, required_materials)
        }) {
        csp
    } else {
        world.build_zones.push_back(build_zone);
        return Worker::Idle(worker);
    };

    let path_from_store_to_build_zone = if let Some(path) = pathfinding::a_star(
        &world.map, //Should I use closest_shop_mut (no _2), this throws an error on borrow rules
        *path_to_store.last().unwrap(),
        build_zone.shop_type.get_non_generic().structure.pos,
    ) {
        path
    } else {
        world.build_zones.push_back(build_zone);
        return Worker::Idle(worker);
    };

    let total_path = combine_path(path_to_store, path_from_store_to_build_zone);

    //make a "reservation" at the store
    take_as_much_as_possible(
        closest_store.get_non_generic_mut().output,
        &mut worker.inventory,
        &build_zone.shop_type.get_build_data().materials_required,
    );

    worker.to_supplying_build_zone(world, total_path, build_zone)
}

///Combines 2 paths, assuming path 2 begins on the same tile as path 1 ends. Removes that repeating
///point.
fn combine_path(
    mut p1: Vec<Pos>,
    mut p2: Vec<Pos>,
) -> Vec<Pos> {
    assert!(p1.last() == p2.first());
    let _ = p1.pop();
    p1.append(&mut p2);
    p1
}

fn has_any_of(
    inv: &Inventory,
    materials: &Vec<InventoryItems>,
) -> bool {
    for (item, _) in materials {
        if inv.get(item) >= 1.0 {
            return true;
        }
    }
    return false;
}

fn take_as_much_as_possible(
    store_inv: &mut Inventory,
    worker_inv: &mut Inventory,
    materials: &Vec<(InventoryItem, f32)>,
) {
    let mut taken_total = 0.0;

    for (item, requested_amount) in materials {
        if taken_total >= worker_inv.limit {
            break;
        }

        let available_in_store = store_inv.get(item);

        if available_in_store <= 0.0 {
            continue; // nothing to take
        }

        let space_left = worker_inv.limit - taken_total;
        let amount_to_take = requested_amount.min(available_in_store).min(space_left);

        if amount_to_take <= 0.0 {
            continue;
        }

        store_inv.remove(item, amount_to_take);
        worker_inv.add(item, amount_to_take);

        taken_total += amount_to_take;
    }
}
