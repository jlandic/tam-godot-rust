pub const WALL: &str = "wall";
pub const FLOOR: &str = "floor";
pub const PLAYER: &str = "player";
pub const EMPTY: &str = "empty";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TileId {
    Wall,
    Floor,
    Player,
    Empty,
}

// TODO: Read mapping from config file, to share w/ Godot
impl TileId {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Wall => WALL,
            Self::Floor => FLOOR,
            Self::Player => PLAYER,
            Self::Empty => EMPTY,
        }
    }

    pub fn iterator() -> impl Iterator<Item = TileId> {
        [Self::Wall, Self::Floor, Self::Player, Self::Empty]
            .iter()
            .copied()
    }
}

impl Default for TileId {
    fn default() -> Self {
        TileId::Empty
    }
}
