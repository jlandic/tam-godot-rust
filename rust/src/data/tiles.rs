pub const WALL: &str = "wall";
pub const FLOOR: &str = "floor";
pub const PLAYER: &str = "player";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

    pub fn iterator() -> impl Iterator<Item = TileId> {
        [Self::Wall, Self::Floor, Self::Player].iter().copied()
    }
}

impl Default for TileId {
    fn default() -> Self {
        TileId::Floor
    }
}
