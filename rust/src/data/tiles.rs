pub const WALL: &str = "wall";
pub const FLOOR: &str = "floor";
pub const PLAYER: &str = "player";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileId {
    Wall,
    Floor,
    Player,
}

// TODO: Read mapping from config file, to share w/ Godot
impl TileId {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Wall => WALL,
            Self::Floor => FLOOR,
            Self::Player => PLAYER,
        }
    }
}
