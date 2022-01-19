use std::cmp::{max, min};

use gdnative::api::*;
use gdnative::prelude::*;

use crate::data::geo::{Map, Rect, Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;

pub struct TestGenerator {
    size: Vec2Unsigned,
    config: TestGeneratorRoomConfig,
    seed: i64,
    rng: Ref<RandomNumberGenerator>,
}

#[derive(Debug, Copy, Clone)]
pub struct TestGeneratorRoomConfig {
    pub max_rooms: i32,
    pub min_room_size: i32,
    pub max_room_size: i32,
}

impl TestGenerator {
    pub fn xy_index(&self, x: i32, y: i32) -> usize {
        Vec2::new(x, y).index(self.size.x as i32)
    }

    pub fn new(size: Vec2Unsigned, seed: i64, config: TestGeneratorRoomConfig) -> Self {
        TestGenerator {
            rng: RandomNumberGenerator::new().into_shared(),
            size,
            seed,
            config,
        }
    }

    pub fn generate(&mut self) -> Map {
        unsafe {
            self.rng.assume_safe().set_seed(self.seed);
        }

        Map {
            tiles: self.new_map(),
            size: self.size,
            ..Default::default()
        }
    }

    fn new_map(&self) -> Vec<TileId> {
        let mut map = vec![TileId::Wall; (self.size.x * self.size.y) as usize];
        let mut rooms: Vec<Rect> = Vec::new();
        let rng = unsafe { self.rng.assume_safe() };

        for _ in 0..self.config.max_rooms {
            let w = rng.randi_range(
                self.config.min_room_size.into(),
                self.config.max_room_size.into(),
            );
            let h = rng.randi_range(
                self.config.min_room_size.into(),
                self.config.max_room_size.into(),
            );
            let x = rng.randi_range(1, self.size.x as i64 - w - 1) - 1;
            let y = rng.randi_range(1, self.size.y as i64 - h - 1) - 1;
            let new_room = Rect::new(x as i32, y as i32, w as i32, h as i32);

            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }

            if ok {
                self.apply_room_to_map(&new_room, &mut map);

                if !rooms.is_empty() {
                    let Vec2 { x: new_x, y: new_y } = new_room.center();
                    let Vec2 {
                        x: prev_x,
                        y: prev_y,
                    } = rooms[rooms.len() - 1].center();

                    if rng.randi_range(0, 2) == 1 {
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        self.apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        self.apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }

                rooms.push(new_room);
            }
        }

        map
    }

    fn apply_room_to_map(&self, room: &Rect, map: &mut [TileId]) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                map[self.xy_index(x, y)] = TileId::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&self, map: &mut [TileId], x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_index(x, y);
            if idx > 0 && idx < (self.size.x * self.size.y) as usize {
                map[idx as usize] = TileId::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&self, map: &mut [TileId], y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_index(x, y);

            if idx > 0 && idx < (self.size.x * self.size.y) as usize {
                map[idx as usize] = TileId::Floor;
            }
        }
    }
}
