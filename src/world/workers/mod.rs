use log::info;
use rand::{rng, seq::IndexedRandom};

use crate::{
    ai::pathfinding,
    math::Pos,
    world::actions::{
        TransitActionResult, gathering_action::GatheringActionResult, taking_break_action::TakingBreakActionResult,
    },
};

use super::{
    actions::{
        ActionResult, BasicAction, TransitAction, gathering_action::GatheringAction,
        taking_break_action::TakingBreakAction,
    },
    inventory::Inventory,
    world_map::WorldMap,
};

pub struct WorkerWithAction<T> {
    pub name: String,

    pub inventory: Inventory,
    pub pos: Pos,

    pub break_progress: BasicAction,

    pub action_data: T,
}

pub struct UnassignedWorkerWithAction<T> {
    pub name: String,

    pub inventory: Inventory,
    pub pos: Pos,

    pub action_data: T,
}

pub struct LostWorker {
    pub name: String,
    pub pos: Pos,
}

pub trait CanReturn {}
pub struct Idle();
pub struct InHearth();
pub struct SupplyingAction(TransitAction);
pub struct StoringAction(TransitAction);
pub struct ReturningAction(TransitAction);

impl CanReturn for SupplyingAction {}
impl CanReturn for StoringAction {}
impl CanReturn for GatheringAction {}
impl CanReturn for TakingBreakAction {}

pub enum Worker {
    //assigned actions
    Idle(WorkerWithAction<Idle>),
    Supplying(WorkerWithAction<SupplyingAction>),
    Storing(WorkerWithAction<StoringAction>),
    Gathering(WorkerWithAction<GatheringAction>),

    Returning(WorkerWithAction<ReturningAction>),

    TakingBreak(WorkerWithAction<TakingBreakAction>),

    ////not assigned actions
    //ReturningToHearth(UnassignedWorkerWithAction<ReturningAction>),
    //InHearth(UnassignedWorkerWithAction<InHearth>),
    //Building,
    //SupplyingBuildZone(UnassignedWorkerWithAction<SupplyAction>),

    ////Maybe? AtS does not implement this action - materials are just magically teleported to the
    ////store if action is cacelled
    //ReturningMaterialsToStore(UnassignedWorkerWithAction<StoringAction>),

    //lost
    Lost(LostWorker),
}

pub enum WorkerActionResult {
    InProgress,
    BroughtToShop(Inventory),
    BroughtToStore(Inventory, Pos),
    Idle,
}

impl Worker {
    pub fn assign<T>(
        unassigned: UnassignedWorkerWithAction<T>,
        map: &WorldMap,
        store_location: Pos,
    ) -> Self {
        if unassigned.inventory.is_empty() {
            let path = if let Some(path) = pathfinding::a_star(map, unassigned.pos, store_location) {
                path
            } else {
                return Worker::Lost(LostWorker {
                    name: unassigned.name.clone(),
                    pos: unassigned.pos,
                });
            };

            return Self::Returning(WorkerWithAction::<ReturningAction> {
                name: unassigned.name,
                inventory: unassigned.inventory,
                pos: unassigned.pos,
                break_progress: BasicAction::new(120.0),
                action_data: ReturningAction(TransitAction::new(path, map)),
            });
        }

        let path = if let Some(path) = pathfinding::dijkstra_closest(map, unassigned.pos, |t| t.is_store()) {
            path
        } else {
            return Self::Lost(LostWorker {
                name: unassigned.name,
                pos: unassigned.pos,
            });
        };

        Self::Storing(WorkerWithAction {
            name: unassigned.name,
            inventory: unassigned.inventory,
            pos: unassigned.pos,
            break_progress: BasicAction::new(120.0),
            action_data: StoringAction(TransitAction::new(path, map)),
        })
    }

    const FIRST_NAMES: [&'static str; 5] = ["Lorien", "Gaud", "Risp", "Horville", "Bargo"];
    const LAST_NAMES: [&'static str; 5] = ["Digger", "Smith", "Miller", "Kowalski", "Laic"];

    pub fn continue_action(
        &mut self,
        delta: f32,
        assigned_store_location: Pos,
        map: &mut WorldMap,
    ) -> WorkerActionResult {
        if let Self::Returning(worker) = self {
            worker.progress_break_requirement(delta);
            let result = worker.action_data.0.continue_action(delta);

            match result {
                TransitActionResult::InProgress(pos) => {
                    worker.pos = pos;
                }
                TransitActionResult::Completed(pos) => {
                    let items = worker.inventory.extract();
                    worker.pos = pos;

                    info!("{} has returned to the shop at {}, and is now idle.", worker.name, pos);
                    if !items.is_empty() {
                        info!("{} has brought the following items to the shop: {}", worker.name, items);
                    }
                    *self = worker.to_idle();
                    return WorkerActionResult::BroughtToShop(items);
                }
            }

            return WorkerActionResult::InProgress;
        }

        if let Self::Storing(worker) = self {
            worker.progress_break_requirement(delta);
            //what if the store was removed? Idc - invoking shop will receive the list of materials
            //this worker had. Could maybe reassign them to this worker and they will be returned
            //to the shop.
            let result = worker.action_data.0.continue_action(delta);

            match result {
                TransitActionResult::InProgress(pos) => {
                    worker.pos = pos;
                }
                TransitActionResult::Completed(pos) => {
                    let items = worker.inventory.extract();
                    worker.pos = pos;

                    info!(
                        "{} has brought items to store at {} and is now returning.",
                        worker.name, pos
                    );

                    *self = worker.to_returning(map, assigned_store_location);
                    return WorkerActionResult::BroughtToStore(items, pos);
                }
            }

            return WorkerActionResult::InProgress;
        }

        if let Self::Supplying(worker) = self {
            worker.progress_break_requirement(delta);
            let result = worker.action_data.0.continue_action(delta);

            match result {
                TransitActionResult::InProgress(pos) => {
                    worker.pos = pos;
                }
                TransitActionResult::Completed(pos) => {
                    worker.pos = pos;

                    info!(
                        "{} has taken reserved items at {} and is now returning.",
                        worker.name, pos
                    );
                    info!("Reserved items: {}", worker.inventory);

                    *self = worker.to_returning(map, assigned_store_location);

                    return WorkerActionResult::InProgress;
                }
            }

            return WorkerActionResult::InProgress;
        }

        if let Self::Gathering(worker) = self {
            worker.progress_break_requirement(delta);
            let result = worker.action_data.continue_action(map, delta);

            match result {
                GatheringActionResult::InProgress(pos) => {
                    worker.pos = pos;
                }
                GatheringActionResult::Completed(mut inv) => {
                    worker.inventory.add_range(inv.drain());

                    info!(
                        "{} has gathered items at {} and is now returning.",
                        worker.name, worker.pos
                    );
                    info!("Gathered items: {}", worker.inventory);

                    *self = worker.to_returning(map, assigned_store_location);

                    return WorkerActionResult::InProgress;
                }
            }
            return WorkerActionResult::InProgress;
        }

        if let Self::TakingBreak(worker) = self {
            let result = worker.action_data.continue_action(delta);

            match result {
                TakingBreakActionResult::InProgress(pos) => {
                    worker.pos = pos;
                }
                TakingBreakActionResult::Completed => {
                    info!(
                        "{} has finished break at {}, and is now returning.",
                        worker.name, worker.pos
                    );
                    worker.break_progress = BasicAction::new(120.0);
                    *self = worker.to_returning(map, assigned_store_location);

                    return WorkerActionResult::InProgress;
                }
            }
            return WorkerActionResult::InProgress;
        }

        if let Self::Idle(worker) = self {
            worker.progress_break_requirement(delta);

            if worker.requires_break() {
                *self = worker.to_taking_break(map);
                return WorkerActionResult::InProgress;
            }

            return WorkerActionResult::Idle;
        }

        if let Self::Lost(_) = self {
            //FIXME: wait a couple of secs and try 'returning_action' again...
            panic!("Pathfinding failed. Worker is lost!");
        }

        unimplemented!("Unassigned actions not yet implemented");
    }
}

impl WorkerWithAction<ReturningAction> {
    fn to_idle(&self) -> Worker {
        Worker::Idle(WorkerWithAction::clone_with(self, Idle {}))
    }
}

impl WorkerWithAction<Idle> {
    fn to_taking_break(
        &mut self,
        map: &WorldMap,
    ) -> Worker {
        info!("{} is starting a break, current pos {}.", self.name, self.pos);
        let path = if let Some(path) = pathfinding::dijkstra_closest(map, self.pos, |t| t.is_hearth()) {
            path
        } else {
            return Worker::Lost(LostWorker {
                name: self.name.clone(),
                pos: self.pos,
            });
        };

        Worker::TakingBreak(WorkerWithAction {
            name: self.name.clone(),
            inventory: self.inventory.clone(),
            pos: self.pos,
            break_progress: self.break_progress.clone(),
            action_data: TakingBreakAction::new(path, map),
        })
    }

    pub fn to_storing(
        &mut self,
        map: &WorldMap,
        path: Vec<Pos>,
        shop_inventory: &mut Inventory,
    ) -> Worker {
        info!("{} is storing materials, current pos {}.", self.name, self.pos);

        info!("{} is storing the following materials {}", self.name, self.inventory);
        shop_inventory.transfer_until_full(&mut self.inventory);

        info!("{} now has the follwoing materials {}", self.name, self.inventory);
        info!("The follwoing materials remain in the shop {}", shop_inventory);

        Worker::Storing(WorkerWithAction::clone_with(
            self,
            StoringAction(TransitAction::new(path, map)),
        ))
    }

    pub fn to_supplying(
        &mut self,
        path: Vec<Pos>,
        map: &WorldMap,
        mut reservation: Inventory,
    ) -> Worker {
        info!("{} is supplying materials, current pos {}.", self.name, self.pos);

        info!(
            "{} is supplying the following materials, which were already reserved: {}",
            self.name, reservation
        );
        let mut new_inv = self.inventory.clone();
        new_inv.add_range(reservation.drain());
        Worker::Supplying(WorkerWithAction {
            name: self.name.clone(),
            inventory: new_inv,
            pos: self.pos,
            break_progress: self.break_progress.clone(),
            action_data: SupplyingAction(TransitAction::new(path, map)),
        })
    }

    pub fn to_gathering(
        &mut self,
        path: Vec<Pos>,
        map: &mut WorldMap,
    ) -> Worker {
        info!(
            "{} is gathering materials at {}, current pos {}.",
            self.name,
            path.last().unwrap(),
            self.pos
        );
        Worker::Gathering(WorkerWithAction::clone_with(self, GatheringAction::new(path, map)))
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanReturn,
{
    fn to_returning(
        &mut self,
        map: &WorldMap,
        assigned_store_location: Pos,
    ) -> Worker {
        let path = if let Some(path) = pathfinding::a_star(map, self.pos, assigned_store_location) {
            path
        } else {
            return Worker::Lost(LostWorker {
                name: self.name.clone(),
                pos: self.pos,
            });
        };

        Worker::Returning(WorkerWithAction::clone_with(
            self,
            ReturningAction(TransitAction::new(path, map)),
        ))
    }
}

impl<T> WorkerWithAction<T> {
    fn clone_with<K>(
        other: &WorkerWithAction<K>,
        action: T,
    ) -> Self {
        Self {
            name: other.name.clone(),
            inventory: other.inventory.clone(),
            pos: other.pos.clone(),
            break_progress: other.break_progress.clone(),
            action_data: action,
        }
    }

    fn progress_break_requirement(
        &mut self,
        delta: f32,
    ) {
        if self.break_progress.is_completed() {
            return;
        }

        self.break_progress.continue_action(delta);
    }

    fn requires_break(&self) -> bool {
        self.break_progress.is_completed()
    }
}
