use crate::{
    ai::pathfinding,
    math::Pos,
    world::{World, structures::ShopType},
};

pub fn closest_shop_mut<'a, F>(
    start: Pos,
    world: &'a mut World,
    f: F,
) -> Option<(&'a mut ShopType, Vec<Pos>)>
where
    F: Fn(&ShopType) -> bool,
{
    let mut stores: Vec<(&mut ShopType, Vec<Pos>)> = world
        .shops
        .iter_mut()
        .filter_map(|s| {
            if f(s) {
                let path = pathfinding::a_star(&world.map, start, s.location());
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

pub fn closest_shop<'a, F>(
    start: Pos,
    world: &'a World,
    f: F,
) -> Option<(&'a ShopType, Vec<Pos>)>
where
    F: Fn(&ShopType) -> bool,
{
    let mut stores: Vec<(&ShopType, Vec<Pos>)> = world
        .shops
        .iter()
        .filter_map(|s| {
            if f(s) {
                let path = pathfinding::a_star(&world.map, start, s.location());
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
