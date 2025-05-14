use std::collections::HashMap;

use log::info;

use crate::{
    math::Pos,
    world::{
        actions::{ActionResult, BasicAction, HaulAction},
        inventory::InventoryItem,
        workers::Worker,
    },
};

#[derive(Clone)]
pub struct Hearth {
    pub pos: Pos,

    pub action: Option<HearthAction>,

    pub inventory: HashMap<InventoryItem, f32>,

    pub hearth_worker: Option<HearthWorker>,
}

#[derive(Clone)]
pub struct HearthWorker {
    pub worker: Worker,
    pub action: Option<HearthWorkerAction>,
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

impl Hearth {
    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 4;

    pub fn new(pos: Pos, worker: Worker) -> Self {
        Self {
            pos,
            action: Some(HearthAction::default()),
            inventory: HashMap::from_iter([(InventoryItem::Wood, 10.0)]),
            hearth_worker: Some(HearthWorker {
                worker,
                action: Some(HearthWorkerAction::default()),
            }),
        }
    }

    pub fn process(&mut self, delta: f32) {
        //TODO: worker should fetch wood if suply low - I will probably need a reference to World
        //since World references this, it might be better to take Store (or whatever) as a
        //reference
        self.process_worker_fuel_haul(delta);

        let action = self.action.take().unwrap();
        match action {
            HearthAction::Burning(action) => self.process_burning(action, delta),
            HearthAction::Idle => self.process_idle(delta),
        };
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

        let action = worker.action.take().unwrap();

        match action {
            HearthWorkerAction::Idle => self.worker_start_hauling(&mut worker),
            HearthWorkerAction::Haul(haul_action) => {
                self.worker_continue_hauling(&mut worker, haul_action, delta)
            }
        }

        self.hearth_worker = Some(worker); //put back worker
    }

    fn worker_start_hauling(&self, worker: &mut HearthWorker) {
        info!("Worker started hauling!");
        worker.action = Some(HearthWorkerAction::Haul(HaulAction {
            progress: 0.0,
            requirement: 20.0,
        }));
    }

    fn worker_continue_hauling(
        &mut self,
        worker: &mut HearthWorker,
        mut action: HaulAction,
        delta: f32,
    ) {
        let result = action.process(delta);
        if let ActionResult::Completed = result {
            info!("Worker finished hauling");
            worker.action = Some(HearthWorkerAction::Idle);

            let wood = self.inventory.get(&InventoryItem::Wood).unwrap_or(&0.0);
            self.inventory.insert(InventoryItem::Wood, wood + 10.0);
            return;
        }

        worker.action = Some(HearthWorkerAction::Haul(action))
    }

    fn process_burning(&mut self, mut action: BasicAction, delta: f32) {
        //note that if I access self.action here... it will be None!
        //that is because only one reference to action can exist, and it is currently in 'action'
        //argument
        let result = action.process(delta);

        if let ActionResult::Completed = result {
            self.action = Some(HearthAction::Idle);
            return;
        }

        //need to put action back
        self.action = Some(HearthAction::Burning(action));
    }

    fn process_idle(&mut self, delta: f32) {
        //this is weird... why do I need to dereference a float?
        let wood = *self.inventory.get(&InventoryItem::Wood).unwrap_or(&0.0);

        //if idle for the first time, this is expected - start new burning process if wood supply
        //allows and worker is present

        if wood > 1.0 && self.hearth_worker.is_some() {
            self.inventory.insert(InventoryItem::Wood, wood - 1.0);

            let burning_action = BasicAction::new(10.0);
            self.action = Some(HearthAction::Burning(burning_action));
            info!("Hearth has started burning, remaining fuel: {}", wood - 1.0);

            return;
        }

        //TODO: bad things should happen if the hearth is not burning
        //for now nothing happens

        self.action = Some(HearthAction::Idle);
    }
}
