use std::collections::{HashMap, LinkedList};

use log::info;

use crate::{
    math::Pos,
    world::{
        World,
        actions::{ActionResult, BasicAction, HaulAction},
        inventory::InventoryItem,
        workers::Worker,
        world_map::WorldMap,
    },
};

use super::{Structure, shop::Shop};

#[derive(Clone)]
pub struct Hearth {
    pub pos: Pos,

    pub action: HearthAction,

    pub inventory: HashMap<InventoryItem, f32>,

    pub hearth_worker: Option<HearthWorker>,
}

#[derive(Clone)]
pub struct HearthWorker {
    pub worker: Worker,
    pub action: HearthWorkerAction,
}

#[derive(Clone, Default)]
pub enum HearthWorkerAction {
    #[default]
    Idle,
    Haul(HaulAction),
}

#[derive(Clone, Default)]
pub enum HearthAction {
    #[default]
    Idle,
    Burning(BasicAction),
}

impl Structure for Hearth {
    fn process(&mut self, map: &mut WorldMap, shops: &LinkedList<Shop>, delta: f32) {
        //TODO: worker should fetch wood if suply low - I will probably need a reference to World
        //since World references this, it might be better to take Store (or whatever) as a
        //reference
        self.process_worker_fuel_haul(delta);

        //TODO: If accessing an enum data, I cannot pass self, as it will contain double reference
        //to the same data. I need to write Self::function(...) and pass all the properties I need,
        //including upacked data. that will definitelly look cleaner.
        match &self.action {
            HearthAction::Burning(_) => Self::process_burning(&mut self.action, delta),
            //HearthAction::Burning(_) => self.process_burning(delta),
            HearthAction::Idle => self.process_idle(delta),
        };
    }
}

impl Hearth {
    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 4;

    pub fn new(pos: Pos, worker: Worker) -> Self {
        Self {
            pos,
            action: HearthAction::default(),
            inventory: HashMap::from_iter([(InventoryItem::Wood, 10.0)]),
            hearth_worker: Some(HearthWorker {
                worker,
                action: HearthWorkerAction::default(),
            }),
        }
    }

    fn process_worker_fuel_haul(&mut self, delta: f32) {
        if self.hearth_worker.is_none() {
            info!("No workers!");
            //cannot fetch fuel
            return;
        }

        if *self.inventory.get(&InventoryItem::Wood).unwrap_or(&0.0) > 10.0 {
            //no need to fetch fuel - stock full
            return;
        }

        let mut worker = self.hearth_worker.take().unwrap();

        match worker.action {
            HearthWorkerAction::Idle => self.worker_start_hauling(&mut worker),
            HearthWorkerAction::Haul(_) => self.worker_continue_hauling(&mut worker, delta),
        }

        self.hearth_worker = Some(worker); //put back worker
    }

    fn worker_start_hauling(&self, worker: &mut HearthWorker) {
        info!("Worker started hauling!");
        worker.action = HearthWorkerAction::Haul(HaulAction {
            progress: 0.0,
            requirement: 20.0,
        });
    }

    fn worker_continue_hauling(&mut self, worker: &mut HearthWorker, delta: f32) {
        if let HearthWorkerAction::Haul(haul_action) = &mut worker.action {
            let result = haul_action.process(delta);

            //finished hauling
            if let ActionResult::Completed = result {
                info!("Worker finished hauling");
                worker.action = HearthWorkerAction::Idle;

                let wood = self.inventory.get(&InventoryItem::Wood).unwrap_or(&0.0);

                //TODO: take from the store

                self.inventory.insert(InventoryItem::Wood, wood + 10.0);
                return;
            }
        } //TODO: all those should have todos... else panic or else error!
    }

    fn process_burning(hearth_action: &mut HearthAction, delta: f32) {
        if let HearthAction::Burning(action) = hearth_action {
            //note that if I access self.action here... it will be None!
            //that is because only one reference to action can exist, and it is currently in 'action'
            //argument
            let result = action.process(delta);

            if let ActionResult::Completed = result {
                *hearth_action = HearthAction::Idle;
                return;
            }
        }
    }

    fn process_idle(&mut self, delta: f32) {
        //this is weird... why do I need to dereference a float?
        let wood = *self.inventory.get(&InventoryItem::Wood).unwrap_or(&0.0);

        //if idle for the first time, this is expected - start new burning process if wood supply
        //allows and worker is present

        if wood > 1.0 && self.hearth_worker.is_some() {
            self.inventory.insert(InventoryItem::Wood, wood - 1.0);

            let burning_action = BasicAction::new(10.0);
            self.action = HearthAction::Burning(burning_action);
            info!("Hearth has started burning, remaining fuel: {}", wood - 1.0);

            return;
        }

        //TODO: bad things should happen if the hearth is not burning
        //for now nothing happens

        self.action = HearthAction::Idle;
    }
}
