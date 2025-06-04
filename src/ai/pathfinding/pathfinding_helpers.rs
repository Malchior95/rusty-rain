use std::collections::LinkedList;

use crate::{
    ai::pathfinding,
    math::Pos,
    world::{World, building::Building, world_map::WorldMap},
};

pub fn closest_shop_mut<'a, F>(
    start: Pos,
    world: &'a mut World,
    f: F,
) -> Option<(&'a mut Building, Vec<Pos>)>
where
    F: Fn(&Building) -> bool,
{
    closest_shop_mut_2(start, &world.map, &mut world.shops, f)
}

pub fn closest_shop<'a, F>(
    start: Pos,
    world: &'a World,
    f: F,
) -> Option<(&'a Building, Vec<Pos>)>
where
    F: Fn(&Building) -> bool,
{
    let mut stores: Vec<(&Building, Vec<Pos>)> = world
        .shops
        .iter()
        .filter_map(|s| {
            if f(s) {
                let path = pathfinding::a_star(&world.map, start, s.building_base.pos);
                path.map(|path| (s, path))
            } else {
                None
            }
        })
        .collect();

    //TODO: compare path's cost, not length
    stores.sort_by(|s1, s2| s2.1.len().cmp(&s1.1.len())); //note reversed comparison order - this
    //is for descending sort

    stores.pop()
}

///The variant of this funciton exists, to make invocation less restrictive on borrow rules.
///Technically, it does not tag WorldMap as mutable, which makes it ok in Rust's eyes. Take a look at
///unassigned_workers.rs to see what I am talking about
pub fn closest_shop_mut_2<'a, F>(
    start: Pos,
    map: &'a WorldMap,
    shops: &'a mut LinkedList<Building>,
    f: F,
) -> Option<(&'a mut Building, Vec<Pos>)>
where
    F: Fn(&Building) -> bool,
{
    let mut stores: Vec<(&mut Building, Vec<Pos>)> = shops
        .iter_mut()
        .filter_map(|s| {
            if f(s) {
                let path = pathfinding::a_star(map, start, s.building_base.pos);
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
