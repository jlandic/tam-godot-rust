use std::ops::{Add, AddAssign};

use gdnative::prelude::Vector2;

use super::directions::Cardinal8;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Self { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn index(&self, width: i32) -> usize {
        (self.y * width + self.x) as usize
    }

    pub fn manhattan_distance(&self, target: &Vec2) -> u32 {
        (self.x - target.x).abs() as u32 + (self.y - target.y).abs() as u32
    }

    pub fn from_index(index: u32, width: u32) -> Self {
        Self {
            x: (index % width) as i32,
            y: (index / width) as i32,
        }
    }

    pub fn chebyshev_distance(&self, to: &Vec2) -> f32 {
        let dx = (self.x - to.x).abs() as f32;
        let dy = (self.y - to.y).abs() as f32;

        // (dx + dy) as f32 + (SQRT_2 - 2.0) * min(dx, dy) as f32
        (dx * dx + dy * dy).sqrt()
    }
}

impl From<Vector2> for Vec2 {
    fn from(godot_vector: Vector2) -> Self {
        Self {
            x: godot_vector.x as i32,
            y: godot_vector.y as i32,
        }
    }
}

impl From<(u32, u32)> for Vec2 {
    fn from(vector: (u32, u32)) -> Self {
        Self {
            x: vector.0 as i32,
            y: vector.1 as i32,
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(vector: (i32, i32)) -> Self {
        Self {
            x: vector.0,
            y: vector.1,
        }
    }
}

impl From<Vec2> for Vector2 {
    fn from(vec: Vec2) -> Self {
        Vector2::new(vec.x as f32, vec.y as f32)
    }
}

impl From<Cardinal8> for Vec2 {
    fn from(direction: Cardinal8) -> Self {
        match direction {
            Cardinal8::North => Vec2::new(0, -1),
            Cardinal8::NorthEast => Vec2::new(1, -1),
            Cardinal8::East => Vec2::new(1, 0),
            Cardinal8::SouthEast => Vec2::new(1, 1),
            Cardinal8::South => Vec2::new(0, 1),
            Cardinal8::SouthWest => Vec2::new(-1, 1),
            Cardinal8::West => Vec2::new(-1, 0),
            Cardinal8::NorthWest => Vec2::new(-1, -1),
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec2Unsigned {
    pub x: u32,
    pub y: u32,
}

impl Vec2Unsigned {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }

    pub fn from_index(index: u32, width: u32) -> Self {
        Self {
            x: index % width,
            y: index / width,
        }
    }

    pub fn index(&self, width: u32) -> u32 {
        self.y * width + self.x
    }
}

impl From<Vec2Unsigned> for Vector2 {
    fn from(vec: Vec2Unsigned) -> Self {
        Vector2::new(vec.x as f32, vec.y as f32)
    }
}

impl Add for Vec2Unsigned {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2Unsigned {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
