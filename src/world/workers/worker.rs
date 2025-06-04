use log::info;

use crate::{
    config::{inventory::InventoryItems, receipes::ProducedReceipe},
    math::Pos,
    world::{
        World,
        actions::{
            ActionResult, TransitActionResult,
            building_action::{BuildingAction, BuildingActionResult},
            gathering_action::{GatheringAction, GatheringActionResult},
            taking_break_action::{TakingBreakAction, TakingBreakActionResult},
        },
        workers::{Worker, worker_with_action::WorkerWithAction},
    },
};

use super::{
    Idle, LostAction, ProducingAction, ReturningAction, StoringAction, SupplyingAction, SupplyingBuildZoneAction,
};

pub enum WorkerActionResult {
    InProgress,
    BroughtToShop(Vec<(InventoryItems, f32)>),
    ProductionComplete(ProducedReceipe),
    Idle,
}

impl Worker {
    pub const TIME_TO_BREAK: f32 = 120.0;

    pub fn continue_action(
        self,
        assigned_shop_pos: Pos,
        delta: f32,
        world: &mut World,
        is_hearth: bool,
    ) -> (Worker, WorkerActionResult) {
        match self {
            Worker::Returning(worker) => handle_returning(worker, delta),
            Worker::Storing(worker) => handle_storing(worker, delta, world, assigned_shop_pos),
            Worker::Supplying(worker) => handle_supplying(worker, delta, world, assigned_shop_pos),
            Worker::Gathering(worker) => handle_gathering(worker, delta, world, assigned_shop_pos),
            Worker::Producing(worker) => handle_producing(worker, delta),
            Worker::TakingBreak(worker) => handle_taking_break(worker, delta, world, assigned_shop_pos),
            Worker::Idle(worker) => handle_idle(worker, delta, world, is_hearth),
            Worker::Lost(worker) => handle_lost(worker, delta, world, assigned_shop_pos),
            //only unassigned
            Worker::SupplyingBuildZone(worker) => handle_supplying_build_zone(worker, delta, world, assigned_shop_pos),
            Worker::Building(worker) => handle_building(worker, delta, world, assigned_shop_pos),
        }
    }
}

fn handle_supplying_build_zone(
    mut worker: WorkerWithAction<SupplyingBuildZoneAction>,
    delta: f32,
    world: &mut World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.0.continue_action(delta);

    match result {
        TransitActionResult::InProgress(pos) => {
            worker.pos = pos;
            (Worker::SupplyingBuildZone(worker), WorkerActionResult::InProgress)
        }
        TransitActionResult::Completed(pos) => {
            worker.pos = pos;
            let (mut worker, mut supplying_action) =
                worker.try_returning_with_action_returned(&world.map, assigned_shop_pos);

            let items = worker.inventory_mut().drain();
            supplying_action.1.materials_delivered.add_range(items);
            world.build_zones.push_back(supplying_action.1);

            (worker, WorkerActionResult::InProgress)
        }
    }
}

fn handle_building(
    mut worker: WorkerWithAction<BuildingAction>,
    delta: f32,
    world: &mut World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.continue_action(world, delta);

    match result {
        BuildingActionResult::InProgress(pos) => {
            worker.pos = pos;
            (Worker::Building(worker), WorkerActionResult::InProgress)
        }
        BuildingActionResult::Completed => {
            info!("{} has completed building.", worker.name);
            (
                worker.try_returning(&world.map, assigned_shop_pos),
                WorkerActionResult::InProgress,
            )
        }
    }
}

fn handle_lost(
    mut worker: WorkerWithAction<LostAction>,
    delta: f32,
    world: &World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let retry_result = worker.action_data.0.continue_action(delta);

    match retry_result {
        ActionResult::InProgress => {
            return (Worker::Lost(worker), WorkerActionResult::InProgress);
        }
        ActionResult::Completed => {
            info!("{} is lost and trying to find a way", worker.name);
            if worker.inventory.is_empty() {
                info!(
                    "{} has empty inventory and is is trying to transition to returning state.",
                    worker.name
                );
                return (
                    worker.try_returning(&world.map, assigned_shop_pos),
                    WorkerActionResult::InProgress,
                );
            }

            info!(
                "{} has non-empty inventory and is trying to transition to storing state.",
                worker.name
            );

            (worker.try_storing(world), WorkerActionResult::InProgress)
        }
    }
}

fn handle_returning(
    mut worker: WorkerWithAction<ReturningAction>,
    delta: f32,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.0.continue_action(delta);

    match result {
        TransitActionResult::InProgress(pos) => {
            worker.pos = pos;
        }
        TransitActionResult::Completed(pos) => {
            worker.pos = pos;

            info!("{} has returned to the shop at {}, and is now idle.", worker.name, pos);
            if !worker.inventory.is_empty() {
                info!(
                    "{} has brought the following items to the shop: {}",
                    worker.name, worker.inventory
                );
            }

            let items: Vec<_> = worker.inventory.drain().collect();
            return (worker.to_idle(), WorkerActionResult::BroughtToShop(items));
        }
    }

    return (Worker::Returning(worker), WorkerActionResult::InProgress);
}

fn handle_storing(
    mut worker: WorkerWithAction<StoringAction>,
    delta: f32,
    world: &mut World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.0.continue_action(delta);

    match result {
        TransitActionResult::InProgress(pos) => {
            worker.pos = pos;
            return (Worker::Storing(worker), WorkerActionResult::InProgress);
        }
        TransitActionResult::Completed(pos) => {
            worker.pos = pos;

            info!(
                "{} has brought items {} to store at {} and is now returning.",
                worker.name, worker.inventory, pos
            );

            let store = if let Some(store) = world
                .shops
                .iter_mut()
                .find(|s| s.building_behaviour.is_store() && s.building_base.pos == worker.pos)
            {
                store
            } else {
                info!(
                    "{} arrived at the store, but the store is missing - searching for new store!",
                    worker.name
                );

                return (worker.to_lost_with_immediate_retry(), WorkerActionResult::InProgress); //being lost will take
                //care of the items in the inventory
            };

            let items: Vec<_> = worker.inventory.drain().collect();

            store.building_base.output.add_range(items);
            worker.inventory.inv.clear();

            return (
                worker.try_returning(&world.map, assigned_shop_pos),
                WorkerActionResult::InProgress,
            );
        }
    }
}

fn handle_supplying(
    mut worker: WorkerWithAction<SupplyingAction>,
    delta: f32,
    world: &mut World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.0.continue_action(delta);

    match result {
        TransitActionResult::InProgress(pos) => {
            worker.pos = pos;
            return (Worker::Supplying(worker), WorkerActionResult::InProgress);
        }
        TransitActionResult::Completed(pos) => {
            worker.pos = pos;

            info!(
                "{} has taken reserved items at {} and is now returning.",
                worker.name, pos
            );
            info!("Reserved items: {}", worker.inventory);

            return (
                worker.try_returning(&world.map, assigned_shop_pos),
                WorkerActionResult::InProgress,
            );
        }
    }
}

fn handle_gathering(
    mut worker: WorkerWithAction<GatheringAction>,
    delta: f32,
    world: &mut World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.continue_action(&mut world.map, delta);

    match result {
        GatheringActionResult::InProgress(pos) => {
            worker.pos = pos;

            return (Worker::Gathering(worker), WorkerActionResult::InProgress);
        }
        GatheringActionResult::Completed(inv) => {
            worker.inventory.add_range(inv);

            info!(
                "{} has gathered items at {} and is now returning.",
                worker.name, worker.pos
            );
            info!("Gathered items: {}", worker.inventory);

            return (
                worker.try_returning(&world.map, assigned_shop_pos),
                WorkerActionResult::InProgress,
            );
        }
    }
}

fn handle_producing(
    mut worker: WorkerWithAction<ProducingAction>,
    delta: f32,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);
    let result = worker.action_data.0.continue_action(delta);

    match result {
        ActionResult::InProgress => {
            return (Worker::Producing(worker), WorkerActionResult::InProgress);
        }
        ActionResult::Completed => {
            info!("{} has completed production.", worker.name);
            let (worker, action_data) = worker.to_idle_with_action_returned();

            return (worker, WorkerActionResult::ProductionComplete(action_data.1));
        }
    }
}

fn handle_taking_break(
    mut worker: WorkerWithAction<TakingBreakAction>,
    delta: f32,
    world: &mut World,
    assigned_shop_pos: Pos,
) -> (Worker, WorkerActionResult) {
    let result = worker.action_data.continue_action(delta);

    match result {
        TakingBreakActionResult::InProgress(pos) => {
            worker.pos = pos;

            return (Worker::TakingBreak(worker), WorkerActionResult::InProgress);
        }
        TakingBreakActionResult::Completed => {
            info!(
                "{} has finished break at {}, and is now returning.",
                worker.name, worker.pos
            );
            worker.break_progress.progress = 0.0;
            worker.exhausted = false;

            return (
                worker.try_returning(&world.map, assigned_shop_pos),
                WorkerActionResult::InProgress,
            );
        }
    }
}

fn handle_idle(
    mut worker: WorkerWithAction<Idle>,
    delta: f32,
    world: &World,
    is_hearth: bool,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);

    if worker.requires_break() {
        return (worker.try_take_break(world, is_hearth), WorkerActionResult::InProgress);
    }

    //TODO: notify stores with result Idle at most once a sec

    return (Worker::Idle(worker), WorkerActionResult::Idle);
}
