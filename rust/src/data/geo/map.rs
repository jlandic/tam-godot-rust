use itertools::Itertools;

use crate::data::geo::{Rect, Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;

use crate::data::geo::directions::CARDINAL_DIRECTIONS;
use crate::data::geo::grid;
use crate::data::geo::{Quadrant, Row};

pub struct Map {
    pub tiles: Vec<TileId>,
    pub tile_entities: Vec<u32>,
    pub revealed: Vec<Vec2>,
    pub visible: Vec<Vec2>,
    pub rooms: Vec<Rect>,
    size: Vec2Unsigned,
}

impl Map {
    pub fn new(size: Vec2Unsigned, tiles: Vec<TileId>) -> Self {
        Self {
            tiles,
            size,
            ..Default::default()
        }
    }

    pub fn is_colliding(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    pub fn is_opaque(&self, position: Vec2) -> bool {
        self.tiles[self.xy_index(position)] == TileId::Wall
    }

    pub fn xy_index(&self, position: Vec2) -> usize {
        position.index(self.size.x as i32) as usize
    }

    pub fn tiles_visible_from(&self, origin: Vec2, range: u32) -> Vec<Vec2> {
        let mut visible: Vec<Vec2> = vec![origin];

        for direction in CARDINAL_DIRECTIONS {
            let quadrant = Quadrant::new(direction, origin);

            let first_row = Row::new(1, -1., 1.);
            visible.extend(self.scan_row_for_visible_tiles(&first_row, &quadrant, range));
        }

        return visible.iter().unique().copied().collect::<Vec<_>>();
    }

    pub fn assign_tiles_visible_from(&mut self, origin: Vec2, range: u32) -> Vec<Vec2> {
        self.visible = self.tiles_visible_from(origin, range);

        self.visible.clone()
    }

    fn scan_row_for_visible_tiles(&self, row: &Row, quadrant: &Quadrant, range: u32) -> Vec<Vec2> {
        let mut visible: Vec<Vec2> = vec![];
        let mut previous_tile_coords: Option<Vec2> = None;

        if range < 1 {
            return visible;
        }

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

                    visible.extend(self.scan_row_for_visible_tiles(&next_row, quadrant, range - 1));
                }
            }

            previous_tile_coords = Some(quadrant.transform(tile.into()));
        }

        if let Some(previous_tile_coords) = previous_tile_coords {
            if !self.is_opaque(previous_tile_coords) {
                visible.extend(self.scan_row_for_visible_tiles(&row.next(), quadrant, range - 1));
            }
        }

        visible
    }

    pub fn reveal_tile(&mut self, position: Vec2) {
        if !self.revealed.iter().contains(&position) {
            self.revealed.push(position);
        }
    }

    pub fn size(&self) -> Vec2Unsigned {
        self.size
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            tiles: Vec::new(),
            size: Vec2Unsigned::zero(),
            tile_entities: Vec::new(),
            revealed: Vec::new(),
            rooms: Vec::new(),
            visible: Vec::new(),
        }
    }
}
