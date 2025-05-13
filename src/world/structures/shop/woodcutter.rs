use crate::world::{
    World,
    actions::{Action, ChopWood, Rest},
    structures::shop::{Inventory, ShopType},
};

use super::Shop;

impl Shop {
    /// This function does noting if a building cannot be placed. True is returned if the operation
    /// is successful. In such a case, the last item of the 'world's shops can be access to get the
    /// woodcutter
    pub fn build_woodcutter(world: &mut World, x: usize, y: usize) -> bool {
        const HEIGHT: u8 = 2;
        const WIDTH: u8 = 2;

        if !world.map.can_build(x, y, 2, 2) {
            return false;
        }

        let woodcutter = Shop {
            workers: Vec::new(),
            inventory: Inventory::default(),
            x,
            y,
            height: HEIGHT,
            width: WIDTH,
            shop_type: ShopType::Woodcutter,
        };

        world.map.build(x, y, WIDTH, HEIGHT);

        world.shops.push_back(woodcutter);

        true
    }

    //FIXME: this logic needs finishing
    pub fn woodcutter_process_update(self, world: &World, delta: f32) {
        for mut worker in self.workers {
            //continue current action
            //
            //Should I use `for worker in &mut self.workers`
            //I need a clone here, cuz I cannot move the value otherwise. Another solution would be
            //that funky Option::Take() That I have seen elsewhere
            worker.action = worker.action.progress();

            //if idle - action was completed. Take a rest or start another action, as decided by
            //the Shop
            if let Action::Idle = worker.action {
                continue;
            }

            //take a break if necessary
            if worker.requries_break() {
                worker.action = Action::Rest(Rest::new());
            }

            //start new actions
            if !self.inventory.is_full() {
                //cut trees
                worker.action = Action::ChopWood(ChopWood::new())
            }
        }
    }
}
