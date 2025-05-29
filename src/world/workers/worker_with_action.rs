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
impl CanReturn for Idle {}
impl CanReturn for LostAction {}

impl CanIdle for ReturningAction {}
impl CanIdle for ProducingAction {}
impl CanIdle for Idle {} //If was trying to transition to a state, but path not found

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
    pub(super) fn to_taking_break(
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

        Worker::Storing(WorkerWithAction::to_new_action(
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
        Worker::Gathering(WorkerWithAction::to_new_action(self, GatheringAction::new(path, map)))
    }

    pub fn to_producing(
        self,
        receipe: &Receipe,
    ) -> Worker {
        Worker::Producing(WorkerWithAction::to_new_action(
            self,
            ProducingAction(BasicAction::new(receipe.requirement), receipe.clone()),
        ))
    }
}

impl<T> WorkerWithAction<T>
where
    T: CanReturn,
{
    pub(super) fn to_returning(
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
    pub(super) fn to_new_action<K>(
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
