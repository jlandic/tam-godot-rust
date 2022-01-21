#[derive(Debug)]
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
