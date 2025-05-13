use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;

use log::info;

pub mod debug_path_drawer;

use crate::data_helpers::with_priority::WithPriority;
use crate::{math::Pos, world::world_map::WorldMap};

pub struct AStarPathFinder<'a> {
    map: &'a WorldMap,
    start: Pos,
    end: Pos,
}

impl<'a> AStarPathFinder<'a> {
    pub fn a_star(&self) -> Option<Vec<Pos>> {
        let start_cost = WithPriority::default(self.start);

        let mut frontier: BinaryHeap<WithPriority<Pos>> = BinaryHeap::new();
        frontier.push(start_cost);

        let mut came_from: HashMap<Pos, Option<Pos>> = HashMap::new();
        came_from.insert(self.start, None);

        let mut cost_so_far: HashMap<Pos, f32> = HashMap::new();
        cost_so_far.insert(self.start, 0.0);

        while !frontier.is_empty() {
            let current = frontier.pop().unwrap().unpack(); //can safely unwrap, the loop will not continue here
            //if frontier is empty

            //found 'end'
            //TODO: handle situation where 'end' is a building - would not be able to move there, but
            //still need to accept the path
            if current == self.end {
                return Some(self.build_path(&came_from));
            }

            //continue search
            for next in self.get_neighbours(current) {
                let cost = cost_so_far[&current] + self.map.get_cost(&next);

                let cost_exists = cost_so_far.contains_key(&next);

                if !cost_exists {
                    cost_so_far.insert(next, cost);
                }

                if !cost_exists || cost < cost_so_far[&next] {
                    cost_so_far.insert(next, cost);

                    let priority = cost + Self::heuristic(&self.end, &next) as f32;
                    frontier.push(WithPriority::new(next, -priority));

                    came_from.insert(next, Some(current));
                }
            }
        }

        None
    }

    fn build_path(&self, came_from: &HashMap<Pos, Option<Pos>>) -> Vec<Pos> {
        info!("Path found, backtracking");
        let mut path = Vec::<Pos>::new();
        let mut current = Some(self.end);
        while current.is_some() {
            let current_pos = current.unwrap();
            path.push(current_pos);
            let next = came_from.get(&current_pos).unwrap();
            current = *next;
        }
        Vec::from_iter(path.iter().copied().rev())
    }

    fn heuristic(a: &Pos, b: &Pos) -> i32 {
        //assumes square grid
        //TODO: explore diagonal movement...
        (a.x as i32 - b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()
    }

    fn get_neighbours(&self, pos: Pos) -> Vec<Pos> {
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
            .filter(|&x| self.within_bounds(x) && self.map.is_traversible(x));

        Vec::from_iter(valid.cloned())
    }

    fn within_bounds(&self, pos: &Pos) -> bool {
        pos.x > 0 && pos.x < self.map.width()
    }
}
