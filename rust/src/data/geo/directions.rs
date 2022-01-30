#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

pub const CARDINAL_DIRECTIONS: [Cardinal; 4] = [
    Cardinal::North,
    Cardinal::East,
    Cardinal::South,
    Cardinal::West,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Cardinal8 {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const CARDINAL_8_DIRECTIONS: [Cardinal8; 8] = [
    Cardinal8::North,
    Cardinal8::NorthEast,
    Cardinal8::East,
    Cardinal8::SouthEast,
    Cardinal8::South,
    Cardinal8::SouthWest,
    Cardinal8::West,
    Cardinal8::NorthWest,
];
