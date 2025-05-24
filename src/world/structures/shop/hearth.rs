use log::info;
use strum::IntoDiscriminant;

use crate::{
    ai::pathfinding::pathfinding_helpers::{self},
    math::Pos,
    world::{
        World,
        actions::{ActionResult, BasicAction},
        inventory::{Inventory, InventoryItem},
        structures::{Shop, ShopType, ShopTypeDiscriminants, Structure},
        workers::{Worker, WorkerActionResult},
        world_map::TileType,
    },
};

pub struct Hearth {
    pub action: HearthAction,
    pub inventory: Inventory, //regular output can be taken from. inventory is private and treated
                              //as input for processing
}

pub enum HearthAction {
    Idle,
    Burning(BasicAction),
}

pub fn build_hearth<'a>(
    world: &'a mut World,
    pos: Pos,
) -> Option<&'a mut Shop<Hearth>> {
    if !world.map.can_build(pos.x, pos.y, Hearth::WIDTH, Hearth::HEIGHT) {
        return None;
    }

    let hearth = Hearth {
        action: HearthAction::Idle,
        inventory: Inventory::from_iter([(InventoryItem::Wood, 10.0)]),
    };

    let shop = Shop {
        structure: Structure {
            pos,
            height: Hearth::HEIGHT,
            width: Hearth::WIDTH,
        },
        workers: Vec::with_capacity(Hearth::MAX_WORKERS as usize),
        max_workers: Hearth::MAX_WORKERS,
        output: Inventory::new(),
        data: hearth,
    };

    let shop_type = ShopType::MainHearth(shop);

    world.map.build(pos.x, pos.y, Hearth::WIDTH, Hearth::HEIGHT, || {
        TileType::Structure(ShopTypeDiscriminants::MainHearth)
    });

    world.shops.push_back(shop_type);

    //return to user for modifications
    if let ShopType::MainHearth(shop) = world.shops.back_mut().unwrap() {
        return Some(shop);
    }
    panic!("std lib failed");
}

impl Hearth {
    pub const WIDTH: u8 = 4;
    pub const HEIGHT: u8 = 4;

    pub const MAX_WORKERS: u8 = 1;

    pub const MATERIAL_SUPPLYING_THRESHOLD: f32 = 10.0;

    //TODO: maybe take workers inventory capacity into accout?
    pub const MIN_MATERIALS_TO_CONSIDER_SUPPLYING: f32 = 5.0;

    pub const WOOD_BURNING_RATE: f32 = 60.0;
}

impl Shop<Hearth> {
    pub fn process(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        self.process_worker_fuel_haul(world, delta);

        let maybe_new_action = match &mut self.data.action {
            HearthAction::Burning(burning) => continue_burning(burning, delta),
            HearthAction::Idle => process_idle(&mut self.data.inventory, !self.workers.is_empty()),
        };

        if let Some(new_action) = maybe_new_action {
            self.data.action = new_action;
        }
    }

    pub fn process_worker_fuel_haul(
        &mut self,
        world: &mut World,
        delta: f32,
    ) {
        for worker in &mut self.workers {
            let mut result = worker.continue_action(delta, self.structure.pos, &mut world.map);

            match result {
                WorkerActionResult::InProgress => {
                    //continue action
                }

                WorkerActionResult::BroughtToStore(_, _) => {
                    //hearth worker does not expect to be bringing anything to store - i.e. it does
                    //not invoke idle_worker.to_storing()
                }

                WorkerActionResult::BroughtToShop(ref mut inventory) => {
                    if inventory.is_empty() {
                        continue;
                    }
                    info!(
                        "The following materials were added to Hearth's inventory: {}",
                        inventory
                    );
                    self.output.add_range(inventory.drain());

                    info!("Hearth has total items: {}", self.output);
                }

                WorkerActionResult::Idle => {
                    if self.data.inventory.get(&InventoryItem::Wood) > Hearth::MATERIAL_SUPPLYING_THRESHOLD {
                        //no need to fetch fuel - stock full
                        continue;
                    }

                    if let Worker::Idle(idle_worker) = worker {
                        let (closest_shop, path) = if let Some(x) =
                            pathfinding_helpers::closest_shop(self.structure.pos, &world.map, &mut world.shops, |s| {
                                s.inventory().get(&InventoryItem::Wood) >= Hearth::MIN_MATERIALS_TO_CONSIDER_SUPPLYING
                            }) {
                            x
                        } else {
                            info!("Hearth has no suitable stores with wood nearby.");
                            continue; //remain idle
                        };

                        let stored_wood = closest_shop.inventory().get(&InventoryItem::Wood);
                        let to_take = f32::min(stored_wood, idle_worker.inventory.limit);

                        closest_shop.inventory_mut().remove(InventoryItem::Wood, to_take);
                        let reservation = Inventory::from_iter([(InventoryItem::Wood, to_take)]);

                        info!(
                            "{} will be supplying {} from {} at {}. Remaining in the store: {}.",
                            idle_worker.name,
                            reservation,
                            closest_shop.discriminant(),
                            path.last().unwrap(),
                            closest_shop.inventory()
                        );

                        *worker = idle_worker.to_supplying(path, &world.map, reservation);
                    }
                }
            }
        }
    }
}

fn continue_burning(
    action: &mut BasicAction,
    delta: f32,
) -> Option<HearthAction> {
    let result = action.continue_action(delta);

    if let ActionResult::Completed = result {
        return Some(HearthAction::Idle);
    }

    None
}

fn process_idle(
    inventory: &mut Inventory,
    has_worker: bool,
    //delta: f32,
) -> Option<HearthAction> {
    let wood = inventory.get(&InventoryItem::Wood);

    if wood > 1.0 && has_worker {
        inventory.remove(InventoryItem::Wood, 1.0);

        let burning_action = BasicAction::new(Hearth::WOOD_BURNING_RATE);
        info!("Hearth has started burning, remaining fuel: {}", wood - 1.0);

        return Some(HearthAction::Burning(burning_action));
    };
    None

    //TODO: bad things should happen if the hearth is not burning
    //for now nothing happens
}
