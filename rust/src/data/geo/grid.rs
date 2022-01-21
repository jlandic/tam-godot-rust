use crate::data::geo::{Cardinal, Vec2};
use std::cell::Cell;

pub type RowTile = (i32, i32);

pub struct Quadrant {
    pub direction: Cardinal,
    pub origin: Vec2,
}

impl Quadrant {
    pub fn new(direction: Cardinal, origin: Vec2) -> Self {
        Self { direction, origin }
    }

    pub fn transform(&self, relative_position: Vec2) -> Vec2 {
        match self.direction {
            Cardinal::North => Vec2::new(
                self.origin.x + relative_position.y,
                self.origin.y - relative_position.x,
            ),
            Cardinal::South => Vec2::new(
                self.origin.x + relative_position.y,
                self.origin.y + relative_position.x,
            ),
            Cardinal::East => Vec2::new(
                self.origin.x + relative_position.x,
                self.origin.y + relative_position.y,
            ),
            Cardinal::West => Vec2::new(
                self.origin.x - relative_position.x,
                self.origin.y + relative_position.y,
            ),
        }
    }
}

pub struct Row {
    pub depth: u32,
    pub slope_start: Cell<f32>,
    pub slope_end: Cell<f32>,
}

impl Row {
    pub fn new(depth: u32, slope_start: f32, slope_end: f32) -> Self {
        Self {
            depth,
            slope_start: Cell::new(slope_start),
            slope_end: Cell::new(slope_end),
        }
    }

    pub fn tiles(&self) -> impl Iterator<Item = RowTile> + '_ {
        let min_x = round_ties_up(self.depth as f32 * self.slope_start.get());
        let max_x = round_ties_down(self.depth as f32 * self.slope_end.get());

        (min_x..=max_x).map(move |x| (self.depth as i32, x))
    }

    pub fn is_tile_symmetric(&self, tile: RowTile) -> bool {
        let (_, col) = tile;

        col as f32 >= self.depth as f32 * self.slope_start.get()
            && col as f32 <= self.depth as f32 * self.slope_end.get()
    }

    pub fn next(&self) -> Self {
        Self::new(self.depth + 1, self.slope_start.get(), self.slope_end.get())
    }
}

pub fn slope(row_tile: RowTile) -> f32 {
    let (depth, col) = row_tile;

    (2 * col - 1) as f32 / (2 * depth) as f32
}

fn round_ties_up(n: f32) -> i32 {
    (n + 0.5).floor() as i32
}

fn round_ties_down(n: f32) -> i32 {
    (n - 0.5).ceil() as i32
}
