use std::collections::LinkedList;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{structures::ShopType, world_map::WorldMap},
};

pub fn closest_shop<'a, F>(
    start: Pos,
    map: &WorldMap,
    shops: &'a mut LinkedList<ShopType>,
    f: F,
) -> Option<(&'a mut ShopType, Vec<Pos>)>
where
    F: Fn(&ShopType) -> bool,
{
    let mut stores: Vec<(&mut ShopType, Vec<Pos>)> = shops
        .iter_mut()
        .filter_map(|s| {
            if f(s) {
                let path = pathfinding::a_star(map, start, s.location());
                path.map(|path| (s, path))
            } else {
                None
            }
        })
        .collect();

    stores.sort_by(|s1, s2| s2.1.len().cmp(&s1.1.len())); //note reversed comparison order - this
    //is for descending sort

    stores.pop()
}
