use pathfinding::prelude::astar;

use crate::data::geo::{Map, Vec2};

pub fn search(map: &Map, start: &Vec2, end: &Vec2) -> Vec<Vec2> {
    if let Some((path, _)) = astar(
        start,
        |p| {
            map.walkable_neighbors(p)
                .iter()
                .map(|p| (*p, 1))
                .collect::<Vec<(Vec2, i32)>>()
        },
        |p| p.manhattan_distance(end) as i32,
        |p| p == end,
    ) {
        path.iter().skip(1).copied().collect()
    } else {
        Vec::new()
    }
}
