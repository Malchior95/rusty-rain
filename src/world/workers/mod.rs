use impl_variant_non_generic::{ImplVariantNonGeneric, IntoNonGeneric};
use log::info;

use crate::{
    ai::pathfinding::{self, pathfinding_helpers},
    math::Pos,
    world::actions::{
        TransitActionResult, gathering_action::GatheringActionResult, taking_break_action::TakingBreakActionResult,
    },
};

use super::{
    World,
    actions::{
        ActionResult, BasicAction, TransitAction, gathering_action::GatheringAction,
        taking_break_action::TakingBreakAction,
    },
    inventory::{Inventory, InventoryItems},
    receipes::Receipe,
    structures::ShopTypeDiscriminants,
    world_map::WorldMap,
};

#[derive(IntoNonGeneric)]
pub struct WorkerWithAction<T> {
    pub name: String,

    pub inventory: Inventory,
    pub pos: Pos,

    pub break_progress: BasicAction,
    pub exhausted: bool,

    pub action_data: T,
}

pub trait CanReturn {}
pub trait CanIdle {}
pub struct Idle();
pub struct InHearth();
pub struct LostAction();
pub struct SupplyingAction(TransitAction);
pub struct StoringAction(TransitAction);
pub struct ReturningAction(TransitAction);
pub struct ProducingAction(BasicAction, Receipe);

impl CanReturn for SupplyingAction {}
impl CanReturn for StoringAction {}
impl CanReturn for GatheringAction {}
impl CanReturn for TakingBreakAction {}
impl CanReturn for Idle {}

impl CanIdle for ReturningAction {}
impl CanIdle for ProducingAction {}
impl CanIdle for Idle {} //If was trying to transition to a state, but path not found

#[derive(ImplVariantNonGeneric)]
pub enum Worker {
    //assigned actions
    Idle(WorkerWithAction<Idle>),
    Supplying(WorkerWithAction<SupplyingAction>),
    Storing(WorkerWithAction<StoringAction>),
    Gathering(WorkerWithAction<GatheringAction>),

    Returning(WorkerWithAction<ReturningAction>),

    TakingBreak(WorkerWithAction<TakingBreakAction>),

    Producing(WorkerWithAction<ProducingAction>),

    ////not assigned actions
    //ReturningToHearth(UnassignedWorkerWithAction<ReturningAction>),
    //InHearth(UnassignedWorkerWithAction<InHearth>),
    //Building,
    //SupplyingBuildZone(UnassignedWorkerWithAction<SupplyAction>),

    ////Maybe? AtS does not implement this action - materials are just magically teleported to the
    ////store if action is cacelled
    //ReturningMaterialsToStore(UnassignedWorkerWithAction<StoringAction>),

    //lost
    Lost(WorkerWithAction<LostAction>),
    //occures when worker was out in the field, but was unable to find his way back to the store
    //TODO: do some BasicAction countdown to retry pathfinding
}

pub enum WorkerActionResult {
    InProgress,
    BroughtToShop(Vec<InventoryItems>),
    ProductionComplete(Receipe),
    Idle,
}

impl Worker {
    pub fn assign<T>(
        unassigned: WorkerWithAction<T>,
        map: &WorldMap,
        store_location: Pos,
    ) -> Self {
        if unassigned.inventory.is_empty() {
            let path = if let Some(path) = pathfinding::a_star(map, unassigned.pos, store_location) {
                path
            } else {
                return Worker::Lost(WorkerWithAction::move_with(unassigned, LostAction()));
            };

            return Self::Returning(WorkerWithAction::move_with(
                unassigned,
                ReturningAction(TransitAction::new(path, map)),
            ));
        }

        let path = if let Some(path) = pathfinding::dijkstra_closest(map, unassigned.pos, |t| t.is_store()) {
            path
        } else {
            return Worker::Lost(WorkerWithAction::move_with(unassigned, LostAction()));
        };

        Self::Storing(WorkerWithAction::move_with(
            unassigned,
            StoringAction(TransitAction::new(path, map)),
        ))
    }

    pub fn continue_action(
        self,
        assigned_shop_pos: Pos,
        assigned_shop_type: ShopTypeDiscriminants,
        delta: f32,
        world: &mut World,
    ) -> (Worker, WorkerActionResult) {
        match self {
            Worker::Returning(mut worker) => {
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

            Worker::Storing(mut worker) => {
                worker.progress_break_requirement(delta);
                //what if the store was removed? Idc - invoking shop will receive the list of materials
                //this worker had. Could maybe reassign them to this worker and they will be returned
                //to the shop.
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
                            //FIXME: No Store found at a location - look for new one
                            panic!();
                        };

                        let items: Vec<_> = worker.inventory.drain().collect();

                        store.get_non_generic_mut().output.add_range(items);
                        worker.inventory.clear();

                        return (
                            worker.to_returning(&mut world.map, assigned_shop_pos),
                            WorkerActionResult::InProgress,
                        );
                    }
                }
            }
            Worker::Supplying(mut worker) => {
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
                            worker.to_returning(&mut world.map, assigned_shop_pos),
                            WorkerActionResult::InProgress,
                        );
                    }
                }
            }
            Worker::Gathering(mut worker) => {
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
                            worker.to_returning(&mut world.map, assigned_shop_pos),
                            WorkerActionResult::InProgress,
                        );
                    }
                }
            }
            Worker::Producing(mut worker) => {
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
            Worker::TakingBreak(mut worker) => {
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
                            worker.to_returning(&mut world.map, assigned_shop_pos),
                            WorkerActionResult::InProgress,
                        );
                    }
                }
            }

            Worker::Idle(mut worker) => {
                worker.progress_break_requirement(delta);

                if worker.requires_break() {
                    return (
                        worker.to_taking_break(world, assigned_shop_type),
                        WorkerActionResult::InProgress,
                    );
                }

                return (Worker::Idle(worker), WorkerActionResult::Idle);
            }
            Worker::Lost(_) => {
                //FIXME: wait a couple of secs and try 'returning_action' again...
                panic!("Pathfinding failed. Worker is lost!");
            }
        }
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanIdle,
{
    fn to_idle(self) -> Worker {
        Worker::Idle(WorkerWithAction::move_with(self, Idle {}))
    }
}

impl WorkerWithAction<Idle> {
    fn to_taking_break(
        mut self,
        world: &World,
        assigned_shop: ShopTypeDiscriminants,
    ) -> Worker {
        //a special case scenario is when hearth tender takes a break. He won't be able to find
        //hearth, as it was removed from the world for processing
        if let ShopTypeDiscriminants::MainHearth = assigned_shop {
            let pos = self.pos;
            return Worker::TakingBreak(WorkerWithAction::move_with(
                self,
                TakingBreakAction::new(vec![pos], &world.map),
            ));
        }

        info!("{} is starting a break, current pos {}.", self.name, self.pos);
        let (_, path) = if let Some(path) = pathfinding_helpers::closest_shop(self.pos, world, |s| s.is_main_hearth()) {
            path
        } else {
            //TODO: if Hearth not accessible - do not became lost
            //In the future: lower the mood or become starving

            self.break_progress.progress = 0.0;
            self.exhausted = true;

            info!(
                "{} was unable to find the Hearth and is now starving/exhausted!",
                self.name
            );

            return self.to_idle();
        };

        Worker::TakingBreak(WorkerWithAction::move_with(
            self,
            TakingBreakAction::new(path, &world.map),
        ))
    }

    pub fn to_storing(
        mut self,
        map: &WorldMap,
        path: Vec<Pos>,
        shop_inventory: &mut Inventory,
    ) -> Worker {
        info!("{} is storing materials, current pos {}.", self.name, self.pos);

        shop_inventory.transfer_until_full(&mut self.inventory);

        info!("{} now has the follwoing materials {}", self.name, self.inventory);
        info!("The follwoing materials remain in the shop {}", shop_inventory);

        Worker::Storing(WorkerWithAction::move_with(
            self,
            StoringAction(TransitAction::new(path, map)),
        ))
    }

    pub fn to_supplying(
        mut self,
        path: Vec<Pos>,
        map: &WorldMap,
        reservation: InventoryItems,
    ) -> Worker {
        info!("{} is supplying materials, current pos {}.", self.name, self.pos);

        info!(
            "{} is supplying the following materials, which were already reserved: {} {}",
            self.name, reservation.0, reservation.1
        );
        self.inventory.add(&reservation.0, reservation.1);
        Worker::Supplying(WorkerWithAction {
            name: self.name,
            inventory: self.inventory,
            pos: self.pos,
            break_progress: self.break_progress,
            exhausted: self.exhausted,
            action_data: SupplyingAction(TransitAction::new(path, map)),
        })
    }

    pub fn to_gathering(
        self,
        path: Vec<Pos>,
        map: &mut WorldMap,
    ) -> Worker {
        info!(
            "{} is gathering materials at {}, current pos {}.",
            self.name,
            path.last().unwrap(),
            self.pos
        );
        Worker::Gathering(WorkerWithAction::move_with(self, GatheringAction::new(path, map)))
    }

    pub fn to_producing(
        self,
        receipe: &Receipe,
    ) -> Worker {
        Worker::Producing(WorkerWithAction::move_with(
            self,
            ProducingAction(BasicAction::new(receipe.requirement), receipe.clone()),
        ))
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanReturn,
{
    fn to_returning(
        self,
        map: &WorldMap,
        assigned_store_location: Pos,
    ) -> Worker {
        let path = if let Some(path) = pathfinding::a_star(map, self.pos, assigned_store_location) {
            path
        } else {
            return Worker::Lost(WorkerWithAction::move_with(self, LostAction()));
        };

        Worker::Returning(WorkerWithAction::move_with(
            self,
            ReturningAction(TransitAction::new(path, map)),
        ))
    }
}

impl<T> WorkerWithAction<T> {
    fn move_with<K>(
        other: WorkerWithAction<K>,
        action: T,
    ) -> Self {
        Self {
            name: other.name,
            inventory: other.inventory,
            pos: other.pos,
            break_progress: other.break_progress,
            exhausted: other.exhausted,
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
