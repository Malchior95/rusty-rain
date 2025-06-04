pub mod build_zone;
pub mod building_behaviour;

use std::collections::LinkedList;

use building_behaviour::{gatherer::GathererBehaviour, hearth::HearthBehaviour, producer::ProducerBehaviour};
use strum_macros::{Display, EnumDiscriminants, EnumIs};

use crate::{config::buildings::Buildings, math::Pos};

use super::{World, inventory::Inventory, worker::Worker};

pub struct BuildingBase {
    pub pos: Pos,
    pub workers: LinkedList<Worker>,
    pub max_workers: u8,
    pub output: Inventory, //todo: really needed here? maybe move to data?
    pub building: Buildings,
}

pub struct Building {
    pub building_base: BuildingBase,
    pub building_behaviour: BuildingBehaviour,
}

//docs to enum dispatch claim that static dispatch can be up to 10x faster than dynamic dispatch,
//due to not having to lookup virtual tables. I am noting this down, cause in the beginning I was
//wondering if it is better to use enums or Box<dyn Trait>

#[derive(EnumIs, EnumDiscriminants)]
#[strum_discriminants(derive(Display))]
pub enum BuildingBehaviour {
    Hearth(HearthBehaviour),
    Store(StoreBehaviour),
    Gatherer(GathererBehaviour),
    Producer(ProducerBehaviour),
}

pub struct StoreBehaviour {}

impl Building {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        match &mut self.building_behaviour {
            BuildingBehaviour::Hearth(hearth) => hearth.process(&mut self.building_base, world, delta),
            BuildingBehaviour::Gatherer(gatherer) => gatherer.process(&mut self.building_base, world, delta),
            BuildingBehaviour::Producer(producer) => producer.process(&mut self.building_base, world, delta),
            _ => {} //currently no update necessary...
        }
    }
}

impl BuildingBehaviourDiscriminants {
    pub fn to_default(&self) -> BuildingBehaviour {
        match self {
            BuildingBehaviourDiscriminants::Hearth => BuildingBehaviour::Hearth(HearthBehaviour::default()),
            BuildingBehaviourDiscriminants::Store => BuildingBehaviour::Store(StoreBehaviour {}),
            BuildingBehaviourDiscriminants::Gatherer => BuildingBehaviour::Gatherer(GathererBehaviour::default()),
            BuildingBehaviourDiscriminants::Producer => BuildingBehaviour::Producer(ProducerBehaviour::default()),
        }
    }
}
