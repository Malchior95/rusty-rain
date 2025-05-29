use impl_variant_non_generic::ImplVariantNonGeneric;
use log::info;

use crate::{
    ai::pathfinding::pathfinding_helpers,
    math::Pos,
    world::{
        World,
        actions::BasicAction,
        actions::{
            ActionResult, TransitAction, TransitActionResult,
            gathering_action::{GatheringAction, GatheringActionResult},
            taking_break_action::{TakingBreakAction, TakingBreakActionResult},
        },
        inventory::InventoryItems,
        receipes::Receipe,
        structures::Shop,
        structures::ShopTypeDiscriminants,
        workers::worker_with_action::{
            Idle, LostAction, ProducingAction, ReturningAction, StoringAction, SupplyingAction, WorkerWithAction,
            WorkerWithActionNonGeneric, WorkerWithActionNonGenericMut,
        },
    },
};

pub mod worker_with_action;

#[derive(ImplVariantNonGeneric)]
pub enum Worker {
    Idle(WorkerWithAction<Idle>),
    Supplying(WorkerWithAction<SupplyingAction>),
    Storing(WorkerWithAction<StoringAction>),
    Gathering(WorkerWithAction<GatheringAction>),
    Returning(WorkerWithAction<ReturningAction>),
    TakingBreak(WorkerWithAction<TakingBreakAction>),
    Producing(WorkerWithAction<ProducingAction>),
    Lost(WorkerWithAction<LostAction>),
    //occures when worker was out in the field, but was unable to find his way back to the store,
    //or when worker was just assigned and is looking for its way to the store
}

pub enum WorkerActionResult {
    InProgress,
    BroughtToShop(Vec<InventoryItems>),
    ProductionComplete(Receipe),
    Idle,
}

impl Worker {
    pub const TIME_TO_BREAK: f32 = 120.0;

    pub fn assign<T, K>(
        shop: &mut Shop<K>,
        unassigned: WorkerWithAction<T>,
    ) {
        //this worker will immediatelly try to find it's shop, maybe first storing it's inventory
        let worker = Worker::Lost(WorkerWithAction::to_new_action(
            unassigned,
            LostAction(BasicAction {
                requirement: 0.0,
                progress: 0.0,
            }),
        ));
        shop.workers.push_back(worker);
    }

    pub fn continue_action(
        self,
        assigned_shop_pos: Pos,
        assigned_shop_type: ShopTypeDiscriminants,
        delta: f32,
        world: &mut World,
    ) -> (Worker, WorkerActionResult) {
        match self {
            Worker::Returning(worker) => handle_returning(worker, delta),
            Worker::Storing(worker) => handle_storing(worker, delta, world, assigned_shop_pos),
            Worker::Supplying(worker) => handle_supplying(worker, delta, world, assigned_shop_pos),
            Worker::Gathering(worker) => handle_gathering(worker, delta, world, assigned_shop_pos),
            Worker::Producing(worker) => handle_producing(worker, delta),
            Worker::TakingBreak(worker) => handle_taking_break(worker, delta, world, assigned_shop_pos),
            Worker::Idle(worker) => handle_idle(worker, delta, world, assigned_shop_type),
            Worker::Lost(worker) => handle_lost(worker, delta, world, assigned_shop_pos),
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
                    worker.to_returning(&world.map, assigned_shop_pos),
                    WorkerActionResult::InProgress,
                );
            }

            info!(
                "{} has non-empty inventory and is trying to transition to storing state.",
                worker.name
            );

            let (_, path) =
                if let Some(path) = pathfinding_helpers::closest_shop(worker.pos, world, |s| s.is_main_store()) {
                    path
                } else {
                    return (
                        Worker::Lost(WorkerWithAction::to_new_action(worker, LostAction::new())),
                        WorkerActionResult::InProgress,
                    );
                };

            (
                Worker::Storing(WorkerWithAction::to_new_action(
                    worker,
                    StoringAction(TransitAction::new(path, &world.map)),
                )),
                WorkerActionResult::InProgress,
            )
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
                .find(|s| s.is_main_store() && s.get_non_generic().structure.pos == worker.pos)
            {
                store
            } else {
                info!(
                    "{} arrived at the store, but the store is missing - searching for new store!",
                    worker.name
                );
                let (_, path) =
                    if let Some(sp) = pathfinding_helpers::closest_shop(worker.pos, world, |s| s.is_main_store()) {
                        sp
                    } else {
                        info!(
                            "{} was looking for another store, but there are no more stores. Becoming lost.",
                            worker.name
                        );
                        return (
                            Worker::Lost(WorkerWithAction::to_new_action(worker, LostAction::new())),
                            WorkerActionResult::InProgress,
                        );
                    };
                return (
                    Worker::Storing(WorkerWithAction::to_new_action(
                        worker,
                        StoringAction(TransitAction::new(path, &world.map)),
                    )),
                    WorkerActionResult::InProgress,
                );
            };

            let items: Vec<_> = worker.inventory.drain().collect();

            store.get_non_generic_mut().output.add_range(items);
            worker.inventory.clear();

            return (
                worker.to_returning(&world.map, assigned_shop_pos),
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
                worker.to_returning(&world.map, assigned_shop_pos),
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
                worker.to_returning(&world.map, assigned_shop_pos),
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
            let receipe = worker.action_data.1.clone();

            return (worker.to_idle(), WorkerActionResult::ProductionComplete(receipe));
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
                worker.to_returning(&world.map, assigned_shop_pos),
                WorkerActionResult::InProgress,
            );
        }
    }
}

fn handle_idle(
    mut worker: WorkerWithAction<Idle>,
    delta: f32,
    world: &World,
    assigned_shop_type: ShopTypeDiscriminants,
) -> (Worker, WorkerActionResult) {
    worker.progress_break_requirement(delta);

    if worker.requires_break() {
        return (
            worker.to_taking_break(world, assigned_shop_type),
            WorkerActionResult::InProgress,
        );
    }

    return (Worker::Idle(worker), WorkerActionResult::Idle);
}
