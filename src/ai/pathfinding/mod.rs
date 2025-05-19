use std::collections::{BinaryHeap, HashMap, LinkedList};

pub mod debug_path_drawer;

use debug_path_drawer::PathDrawer;
use log::info;

use crate::data_helpers::with_priority::WithPriority;
use crate::world::world_map::TileType;
use crate::{math::Pos, world::world_map::WorldMap};

const HEURISTICS_INFLUENCE: f32 = 0.5;

pub fn breadth_first_closest(
    map: &WorldMap,
    start: Pos,
    tile_type: &TileType,
    exact: bool,
    include_non_traversible: bool,
) -> Option<Vec<Pos>> {
    let mut frontier: LinkedList<Pos> = LinkedList::new();
    frontier.push_front(start);

    let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
    came_from.insert(start, None);

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();
        //if frontier is empty

        //found 'end'
        //TODO: handle situation where 'end' is a building - would not be able to move there, but
        //still need to accept the path
        if map.get(&current).is_match(tile_type, exact) {
            return Some(build_path(&came_from, current, map));
        }

        //continue search
        for next in get_neighbours(map, &current, include_non_traversible) {
            if !came_from.contains_key(&next) {
                frontier.push_back(next);

                came_from.insert(next, Some(current));
            }
        }
    }

    None
}

pub fn a_star(map: &WorldMap, start: Pos, end: Pos) -> Option<Vec<Pos>> {
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
        //TODO: handle situation where 'end' is a building - would not be able to move there, but
        //still need to accept the path
        if current == end {
            return Some(build_path(&came_from, end, map));
        }

        //continue search
        for next in get_neighbours(map, &current, false) {
            let cost = cost_so_far[&current] + map.get_cost(&next);

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

fn build_path(came_from: &HashMap<Pos, Option<Pos>>, end: Pos, map: &WorldMap) -> Vec<Pos> {
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

fn heuristic(a: &Pos, b: &Pos) -> f32 {
    //assumes square grid
    //TODO: explore diagonal movement...
    let base = (a.x as f32 - b.x as f32).abs() + (a.y as f32 - b.y as f32).abs();
    base * HEURISTICS_INFLUENCE
}

fn get_neighbours(map: &WorldMap, pos: &Pos, include_non_traversible: bool) -> Vec<Pos> {
    let ret = [
        Pos {
            x: pos.x - 1,
            y: pos.y,
        },
        Pos {
            x: pos.x,
            y: pos.y + 1,
        },
        Pos {
            x: pos.x + 1,
            y: pos.y,
        },
        Pos {
            x: pos.x,
            y: pos.y - 1,
        },
    ];
    let valid = ret
        .iter()
        .filter(|&x| map.within_bounds(x) && (map.is_traversible(x) || include_non_traversible));

    valid.cloned().collect()
}
