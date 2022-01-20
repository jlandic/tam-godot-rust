use crate::data::geo::{Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;

pub struct Map {
    pub tiles: Vec<TileId>,
    pub size: Vec2Unsigned,
    pub tile_entities: Vec<u32>,
}

impl Map {
    pub fn is_colliding(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    #[allow(dead_code)]
    pub fn is_opaque(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    pub fn xy_index(&self, position: Vec2) -> usize {
        position.index(self.size.x as i32) as usize
    }

    pub fn square(&self, origin: Vec2, radius: i32) -> Vec<Vec2> {
        let mut points = vec![origin];
        for i in -radius..=radius {
            for j in -radius..=radius {
                let point = Vec2::new(origin.x + i, origin.y + j);

                if point.x >= 0
                    && point.y >= 0
                    && point.x < self.size.x as i32
                    && point.y < self.size.y as i32
                {
                    points.push(point);
                }
            }
        }

        points
    }

    pub fn first_free_position(&self) -> Option<Vec2> {
        (0..self.tiles.len())
            .find(|i| {
                let position = Vec2::from_index(*i as u32, self.size.x);
                !self.is_colliding(position)
            })
            .map(|i| Vec2::from_index(i as u32, self.size.x))
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: Vec::new(),
            size: Vec2Unsigned::zero(),
            tile_entities: Vec::new(),
        }
    }
}
