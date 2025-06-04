use crate::{
    config::receipes::ProducedReceipe,
    math::Pos,
    world::{
        actions::{gathering_action::GatheringAction, taking_break_action::TakingBreakAction},
        workers::worker_with_action::WorkerWithAction,
    },
};

use super::{
    actions::{
        BasicAction,
        TransitAction, //supplying_build_zone_action::SupplyingBuildZoneAction,
        building_action::BuildingAction,
    },
    inventory::Inventory,
    structures::build_zone::BuildZone,
};

pub mod unassigned_workers;
pub mod worker;
pub mod worker_with_action;

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

    //unassigned only actions
    SupplyingBuildZone(WorkerWithAction<SupplyingBuildZoneAction>),
    Building(WorkerWithAction<BuildingAction>),
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
pub struct ProducingAction(pub BasicAction, pub ProducedReceipe);
pub struct SupplyingBuildZoneAction(pub TransitAction, pub BuildZone);

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

impl CanGetLost for SupplyingBuildZoneAction {}
impl CanGetLost for BuildingAction {}
impl CanReturn for SupplyingBuildZoneAction {}
impl CanReturn for BuildingAction {}

//use #![feature(macro_metavar_expr_concat)] once that becomes stable
use paste::paste;
macro_rules! worker_impl {
    ($name:ident, $type:ty) => {
        paste! {
        impl Worker {
            pub fn $name(&self) -> &$type {
                match self {
                    Worker::Idle(w) => &w.$name,
                    Worker::Supplying(w) => &w.$name,
                    Worker::Storing(w) => &w.$name,
                    Worker::Gathering(w) => &w.$name,
                    Worker::Returning(w) => &w.$name,
                    Worker::TakingBreak(w) => &w.$name,
                    Worker::Producing(w) => &w.$name,
                    Worker::Lost(w) => &w.$name,
                    Worker::SupplyingBuildZone(w) => &w.$name,
                    Worker::Building(w) => &w.$name,
                }
            }

            pub fn [<$name _mut>](&mut self) -> &mut $type {
                match self {
                    Worker::Idle(w) => &mut w.$name,
                    Worker::Supplying(w) => &mut w.$name,
                    Worker::Storing(w) => &mut w.$name,
                    Worker::Gathering(w) => &mut w.$name,
                    Worker::Returning(w) => &mut w.$name,
                    Worker::TakingBreak(w) => &mut w.$name,
                    Worker::Producing(w) => &mut w.$name,
                    Worker::Lost(w) => &mut w.$name,
                    Worker::SupplyingBuildZone(w) => &mut w.$name,
                    Worker::Building(w) => &mut w.$name,
                }
            }
        }
        }
    };
}

worker_impl!(pos, Pos);
worker_impl!(inventory, Inventory);
worker_impl!(name, String);
worker_impl!(break_progress, BasicAction);
worker_impl!(exhausted, bool);
