use impl_variant_non_generic::ImplVariantNonGeneric;

use crate::world::{
    actions::{gathering_action::GatheringAction, taking_break_action::TakingBreakAction},
    receipes::Receipe,
    workers::worker_with_action::{WorkerWithAction, WorkerWithActionNonGeneric, WorkerWithActionNonGenericMut},
};

use super::{
    actions::{
        BasicAction,
        TransitAction,
        building_action::BuildingAction,
        //supplying_build_zone_action::SupplyingBuildZoneAction,
    },
    structures::build_zone::BuildZone,
};

pub mod unassigned_workers;
pub mod worker;
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
pub struct ProducingAction(pub BasicAction, pub Receipe);
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
