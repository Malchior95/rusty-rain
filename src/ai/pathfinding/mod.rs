use std::collections::{BinaryHeap, HashMap};

pub mod debug_path_drawer;
pub mod pathfinding_helpers;

use debug_path_drawer::PathDrawer;
use log::info;

use crate::data_helpers::with_priority::WithPriority;
use crate::world::world_map::TileType;
use crate::{math::Pos, world::world_map::WorldMap};

const HEURISTICS_INFLUENCE: f32 = 0.5;

pub fn dijkstra_closest<F>(
    map: &WorldMap,
    start: Pos,
    tile_type_check: F,
) -> Option<Vec<Pos>>
where
    F: Fn(&TileType) -> bool,
{
    if tile_type_check(map.get(&start)) {
        return Some(Vec::from_iter([start]));
    }

    let start_cost = WithPriority::default(start);

    let mut frontier: BinaryHeap<WithPriority<Pos>> = BinaryHeap::new();
    frontier.push(start_cost);

    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(start, None);

    let mut cost_so_far: HashMap<Pos, f32> = HashMap::new();
    cost_so_far.insert(start, 0.0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap().unpack();
        //if frontier is empty

        //found 'end'
        if let Some(final_tile) = get_nearby(map, &current, &tile_type_check) {
            came_from.insert(final_tile, Some(current));
            return Some(build_path(&came_from, final_tile, map));
        }

        //continue search
        for next in get_neighbours(map, &current) {
            let cost = cost_so_far[&current] + map.get(&next).cost();

            let cost_exists = cost_so_far.contains_key(&next);

            if !cost_exists {
                cost_so_far.insert(next, cost);
            }

            if !cost_exists || cost < cost_so_far[&next] {
                cost_so_far.insert(next, cost);

                let priority = cost;
                frontier.push(WithPriority::new(next, -priority));

                came_from.insert(next, Some(current));
            }
        }
    }

    None
}

pub fn a_star(
    map: &WorldMap,
    start: Pos,
    end: Pos,
) -> Option<Vec<Pos>> {
    if start == end {
        return Some(Vec::from_iter([start]));
    }

    let start_cost = WithPriority::default(start);

    let mut frontier: BinaryHeap<WithPriority<Pos>> = BinaryHeap::new();
    frontier.push(start_cost);

    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(start, None);

    let mut cost_so_far: HashMap<Pos, f32> = HashMap::new();
    cost_so_far.insert(start, 0.0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap().unpack(); //can safely unwrap, the loop will not continue here
        //if frontier is empty

        //found 'end'
        if is_nearby(map, &current, &end) {
            came_from.insert(end, Some(current));
            return Some(build_path(&came_from, end, map));
        }

        //continue search
        for next in get_neighbours(map, &current) {
            let cost = cost_so_far[&current] + map.get(&next).cost();

            let cost_exists = cost_so_far.contains_key(&next);

            if !cost_exists {
                cost_so_far.insert(next, cost);
            }

            if !cost_exists || cost < cost_so_far[&next] {
                cost_so_far.insert(next, cost);

                let priority = cost + heuristic(&end, &next);
                frontier.push(WithPriority::new(next, -priority));

                came_from.insert(next, Some(current));
            }
        }
    }

    None
}

fn build_path(
    came_from: &HashMap<Pos, Option<Pos>>,
    end: Pos,
    map: &WorldMap,
) -> Vec<Pos> {
    let mut path = Vec::<Pos>::new();
    let mut current = Some(end);
    while current.is_some() {
        let current_pos = current.unwrap();
        path.push(current_pos);
        let next = came_from.get(&current_pos).unwrap();
        current = *next;
    }
    let path = path.iter().copied().rev().collect();

    let map_drawer = PathDrawer { map, path: &path };

    info!("\n{}", map_drawer);

    path
}

fn heuristic(
    a: &Pos,
    b: &Pos,
) -> f32 {
    //assumes square grid
    //TODO: explore diagonal movement...
    let base = (a.x as f32 - b.x as f32).abs() + (a.y as f32 - b.y as f32).abs();
    base * HEURISTICS_INFLUENCE
}

fn get_neighbours(
    map: &WorldMap,
    pos: &Pos,
) -> Vec<Pos> {
    let bottom = if pos.y > 0 {
        Some(Pos::new(pos.x, pos.y - 1))
    } else {
        None
    };
    let top = if pos.y < map.height() - 1 {
        Some(Pos::new(pos.x, pos.y + 1))
    } else {
        None
    };
    let left = if pos.x > 0 {
        Some(Pos::new(pos.x - 1, pos.y))
    } else {
        None
    };
    let right = if pos.x < map.width() - 1 {
        Some(Pos::new(pos.x + 1, pos.y))
    } else {
        None
    };

    let ret = [bottom, left, top, right];
    let valid = ret
        .iter()
        .filter_map(|p| p.as_ref())
        .filter(|&x| (map.get(x).is_traversible()));

    valid.cloned().collect()
}

fn get_nearby<F>(
    map: &WorldMap,
    pos: &Pos,
    tile_type_check: F,
) -> Option<Pos>
where
    F: Fn(&TileType) -> bool,
{
    let bottom = if pos.y > 0 {
        Some(Pos::new(pos.x, pos.y - 1))
    } else {
        None
    };
    let top = if pos.y < map.height() - 1 {
        Some(Pos::new(pos.x, pos.y + 1))
    } else {
        None
    };
    let left = if pos.x > 0 {
        Some(Pos::new(pos.x - 1, pos.y))
    } else {
        None
    };
    let right = if pos.x < map.width() - 1 {
        Some(Pos::new(pos.x + 1, pos.y))
    } else {
        None
    };

    let ret = [bottom, left, top, right];
    ret.iter()
        .filter_map(|p| p.as_ref())
        .filter(|t| tile_type_check(map.get(*t)))
        .nth(0)
        .copied()
}
fn is_nearby(
    map: &WorldMap,
    pos: &Pos,
    target: &Pos,
) -> bool
where
{
    let bottom = if pos.y > 0 {
        Some(Pos::new(pos.x, pos.y - 1))
    } else {
        None
    };
    let top = if pos.y < map.height() - 1 {
        Some(Pos::new(pos.x, pos.y + 1))
    } else {
        None
    };
    let left = if pos.x > 0 {
        Some(Pos::new(pos.x - 1, pos.y))
    } else {
        None
    };
    let right = if pos.x < map.width() - 1 {
        Some(Pos::new(pos.x + 1, pos.y))
    } else {
        None
    };

    let ret = [bottom, left, top, right];
    ret.iter()
        .filter_map(|p| p.as_ref())
        .filter(|&p| p == target)
        .any(|_| true)
}
