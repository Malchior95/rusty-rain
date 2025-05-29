use std::collections::LinkedList;

use rusty_rain::{
    math::Pos,
    world::{
        World,
        world_map::{TileType, WorldMap, resources::ResourceType},
    },
};

pub fn new_test_map(
    width: usize,
    height: usize,
) -> WorldMap {
    let mut world = WorldMap::new(width, height);

    //create boundary of trees
    for y in 0..height {
        for x in 0..width {
            if [0, 1, height - 2, height - 1].contains(&y) || [0, 1, width - 2, width - 1].contains(&x) {
                world.map[y][x] = ResourceType::tile_tree();
            }
        }
    }

    world
}

pub fn new_test_world(
    width: usize,
    height: usize,
) -> World {
    let mut map = new_test_map(width, height);

    //draw road
    (3..7)
        .map(|y| Pos::new(3, y))
        .for_each(|p| map.map[p.y][p.x] = TileType::Road);
    (3..8)
        .map(|x| Pos::new(x, 6))
        .for_each(|p| map.map[p.y][p.x] = TileType::Road);

    //plant berry bush
    *map.get_mut(&Pos::new(3, 12)) = ResourceType::tile_berry();

    //plant tree
    *map.get_mut(&Pos::new(3, 7)) = ResourceType::tile_tree();

    World {
        map,
        shops: LinkedList::new(),
        unassigned_workers: LinkedList::new(),
        frame_number: 0,
    }
}
