use crate::{
    ai::pathfinding::{self, pathfinding_helpers},
    math::Pos,
    world::{
        World,
        actions::{
            BasicAction, TransitAction, gathering_action::GatheringAction, taking_break_action::TakingBreakAction,
        },
        inventory::{Inventory, InventoryItems},
        receipes::Receipe,
        structures::ShopTypeDiscriminants,
        world_map::WorldMap,
    },
};
use impl_variant_non_generic::IntoNonGeneric;
use log::info;

use super::Worker;

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
pub trait CanStore {}
pub trait CanGetLost {}

pub struct Idle();
pub struct InHearth();
pub struct LostAction(pub BasicAction);
pub struct SupplyingAction(pub TransitAction);
pub struct StoringAction(pub TransitAction);
pub struct ReturningAction(pub TransitAction);
pub struct ProducingAction(pub BasicAction, pub Receipe);

impl CanReturn for SupplyingAction {}
impl CanReturn for StoringAction {}
impl CanReturn for GatheringAction {}
impl CanReturn for TakingBreakAction {}
impl CanReturn for LostAction {}

impl CanIdle for ReturningAction {} //returned to the shop
impl CanIdle for ProducingAction {} //finished production at the shop
impl CanIdle for Idle {} //Still idle or, If was trying to transition to a state, but path not found

impl CanStore for Idle {}
impl CanStore for LostAction {} //bring whatever is in the inventory to store before attempting to
//come back to the shop

impl CanGetLost for LostAction {} //still lost
impl CanGetLost for StoringAction {} //was trying to return, but got lost
impl CanGetLost for SupplyingAction {} //was trying to return, but got lost
impl CanGetLost for GatheringAction {} //was trying to return, but got lost
impl CanGetLost for TakingBreakAction {} //was trying to return, but got lost

impl<T> WorkerWithAction<T>
where
    T: CanGetLost,
{
    pub fn to_lost(self) -> Worker {
        Worker::Lost(WorkerWithAction::to_new_action(self, LostAction::new()))
    }

    pub fn to_lost_with_immediate_retry(self) -> Worker {
        let mut worker = WorkerWithAction::to_new_action(self, LostAction::new());
        worker.break_progress.progress = LostAction::RETRY_DELAY;
        Worker::Lost(worker)
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanIdle,
{
    pub(super) fn to_idle(self) -> Worker {
        Worker::Idle(WorkerWithAction::to_new_action(self, Idle {}))
    }
}

impl LostAction {
    pub const RETRY_DELAY: f32 = 15.0;
    pub fn new() -> Self {
        Self(BasicAction::new(Self::RETRY_DELAY))
    }
}

impl WorkerWithAction<Idle> {
    pub fn try_storing(
        self,
        world: &World,
    ) -> Worker {
        let (_, path) = if let Some(path) = pathfinding_helpers::closest_shop(self.pos, world, |s| s.is_main_store()) {
            path
        } else {
            return self.to_idle();
        };

        self.to_storing(&world.map, path)
    }
}

impl WorkerWithAction<LostAction> {
    pub(super) fn try_storing(
        self,
        world: &World,
    ) -> Worker {
        let (_, path) = if let Some(path) = pathfinding_helpers::closest_shop(self.pos, world, |s| s.is_main_store()) {
            path
        } else {
            return self.to_lost();
        };

        self.to_storing(&world.map, path)
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanStore,
{
    /// If Idle worker cannot transition to storing, they should remain idle. If lost, they
    /// should remain lost
    fn to_storing(
        self,
        map: &WorldMap,
        path: Vec<Pos>,
    ) -> Worker {
        Worker::Storing(WorkerWithAction::to_new_action(
            self,
            StoringAction(TransitAction::new(path, map)),
        ))
    }
}

impl WorkerWithAction<Idle> {
    pub(super) fn try_take_break(
        mut self,
        world: &World,
        assigned_shop: ShopTypeDiscriminants,
    ) -> Worker {
        info!("{} is starting a break, current pos {}.", self.name, self.pos);

        //a special case scenario is when hearth tender takes a break. He won't be able to find
        //hearth, as it was removed from the world for processing
        if let ShopTypeDiscriminants::MainHearth = assigned_shop {
            let pos = self.pos;
            return Worker::TakingBreak(WorkerWithAction::to_new_action(
                self,
                TakingBreakAction::new(vec![pos], &world.map),
            ));
        }

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

        Worker::TakingBreak(WorkerWithAction::to_new_action(
            self,
            TakingBreakAction::new(path, &world.map),
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
        Worker::Supplying(WorkerWithAction::to_new_action(
            self,
            SupplyingAction(TransitAction::new(path, map)),
        ))
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
        Worker::Gathering(WorkerWithAction::to_new_action(self, GatheringAction::new(path, map)))
    }

    pub fn to_producing(
        self,
        receipe: Receipe,
    ) -> Worker {
        Worker::Producing(WorkerWithAction::to_new_action(
            self,
            ProducingAction(BasicAction::new(receipe.requirement), receipe),
        ))
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanReturn,
{
    pub(super) fn try_returning(
        self,
        map: &WorldMap,
        assigned_shop_pos: Pos,
    ) -> Worker {
        let path = if let Some(path) = pathfinding::a_star(map, self.pos, assigned_shop_pos) {
            path
        } else {
            info!("{} was not able to find a way to the shop!", self.name);
            return Worker::Lost(WorkerWithAction::to_new_action(self, LostAction::new()));
        };

        Worker::Returning(WorkerWithAction::to_new_action(
            self,
            ReturningAction(TransitAction::new(path, map)),
        ))
    }
}

impl<T> WorkerWithAction<T> {
    fn to_new_action<K>(
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

    pub(super) fn progress_break_requirement(
        &mut self,
        delta: f32,
    ) {
        if self.break_progress.is_completed() {
            return;
        }

        self.break_progress.continue_action(delta);
    }

    pub(super) fn requires_break(&self) -> bool {
        self.break_progress.is_completed()
    }
}
