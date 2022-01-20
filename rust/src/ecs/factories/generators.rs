use crate::{
    data::geo::Vec2Unsigned,
    generators::{TestGenerator, TestGeneratorRoomConfig},
};

pub struct TestGeneratorFactory;

impl TestGeneratorFactory {
    pub fn default() -> TestGenerator {
        TestGenerator::new(
            Vec2Unsigned::new(16 * 4, 9 * 4),
            43,
            TestGeneratorRoomConfig {
                min_room_size: 5,
                max_room_size: 10,
                max_rooms: 15,
            },
        )
    }
}
