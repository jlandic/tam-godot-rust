use std::ops::{Add, AddAssign};

use gdnative::prelude::Vector2;

use crate::data::geo::directions::Cardinal;

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

    pub fn from_index(index: u32, width: u32) -> Self {
        Self {
            x: (index % width) as i32,
            y: (index / width) as i32,
        }
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

impl From<Cardinal> for Vec2 {
    fn from(direction: Cardinal) -> Self {
        let x = match direction {
            Cardinal::North | Cardinal::South => 0,
            Cardinal::East | Cardinal::West => 1,
        };
        let y = match direction {
            Cardinal::North | Cardinal::South => 1,
            Cardinal::East | Cardinal::West => 0,
        };

        Self { x, y }
    }
}

impl From<Vec2> for Vector2 {
    fn from(vec: Vec2) -> Self {
        Vector2::new(vec.x as f32, vec.y as f32)
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
