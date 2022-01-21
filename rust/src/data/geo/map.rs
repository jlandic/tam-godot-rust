use gdnative::prelude::*;
use itertools::Itertools;

use crate::data::geo::{Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;

use crate::data::geo::directions::CARDINAL_DIRECTIONS;
use crate::data::geo::grid;
use crate::data::geo::{Quadrant, Row};

pub struct Map {
    pub tiles: Vec<TileId>,
    pub size: Vec2Unsigned,
    pub tile_entities: Vec<u32>,
}

impl Map {
    pub fn is_colliding(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    pub fn is_opaque(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    pub fn xy_index(&self, position: Vec2) -> usize {
        position.index(self.size.x as i32) as usize
    }

    pub fn first_free_position(&self) -> Option<Vec2> {
        (0..self.tiles.len())
            .find(|i| {
                let position = Vec2::from_index(*i as u32, self.size.x);
                !self.is_colliding(position)
            })
            .map(|i| Vec2::from_index(i as u32, self.size.x))
    }

    pub fn tiles_visible_from(&self, origin: Vec2) -> Vec<Vec2> {
        let mut visible: Vec<Vec2> = vec![origin];

        for direction in CARDINAL_DIRECTIONS {
            godot_print!("{:?}", direction);
            let quadrant = Quadrant::new(direction, origin);

            let first_row = Row::new(1, -1., 1.);
            visible.extend(self.scan_row_for_visible_tiles(&first_row, &quadrant));
        }

        return visible.iter().unique().copied().collect::<Vec<_>>();
    }

    fn scan_row_for_visible_tiles(&self, row: &Row, quadrant: &Quadrant) -> Vec<Vec2> {
        let mut visible: Vec<Vec2> = vec![];
        let mut previous_tile_coords: Option<Vec2> = None;

        for tile in row.tiles() {
            let coords = quadrant.transform(tile.into());

            if self.is_opaque(coords) || row.is_tile_symmetric(tile) {
                visible.push(quadrant.transform(tile.into()));
            }
            if let Some(previous_tile_coords) = previous_tile_coords {
                if self.is_opaque(previous_tile_coords) && !self.is_opaque(coords) {
                    row.slope_start.set(grid::slope(tile));
                }
                if !self.is_opaque(previous_tile_coords) && self.is_opaque(coords) {
                    let next_row = row.next();
                    next_row.slope_end.set(grid::slope(tile));

                    visible.extend(self.scan_row_for_visible_tiles(&next_row, quadrant));
                }
            }

            previous_tile_coords = Some(quadrant.transform(tile.into()));
        }

        if let Some(previous_tile_coords) = previous_tile_coords {
            if !self.is_opaque(previous_tile_coords) {
                visible.extend(self.scan_row_for_visible_tiles(&row.next(), quadrant));
            }
        }

        visible
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
