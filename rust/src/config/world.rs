const DEFAULT_TILE_SIZE: u32 = 16;

pub struct WorldConfig {
    pub tile_size: u32,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            tile_size: DEFAULT_TILE_SIZE,
        }
    }
}
