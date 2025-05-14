use crate::world::{
    World,
    actions::{ChopWood, Rest},
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
}
