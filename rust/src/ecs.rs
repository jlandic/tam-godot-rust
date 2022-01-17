use gdnative::api::*;
use gdnative::prelude::*;
use hecs::World;

use crate::config::WorldConfig;
use crate::data::geo::{Map, Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;
use crate::ecs::messages::TileChanged;
use crate::engine::SceneSpawner;
use crate::generators::{TestGenerator, TestGeneratorRoomConfig};

use self::{
    components::{Player, Position, Tile},
    messages::Moved,
};

mod components;
mod events;
mod messages;
mod sync;
mod systems;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Ecs {
    world: World,
    world_config: WorldConfig,
    #[property]
    base_scene: Ref<PackedScene>,
    resources: Resources,
}

#[derive(Default)]
pub struct Resources {
    pub map: Map,
}

#[methods]
impl Ecs {
    fn new(_owner: &Node) -> Self {
        Self {
            world: World::new(),
            world_config: WorldConfig::default(),
            base_scene: PackedScene::new().into_shared(),
            resources: Resources::default(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        let mut test_generator = TestGenerator::new(
            Vec2Unsigned::new(16 * 4, 9 * 4),
            42,
            TestGeneratorRoomConfig {
                min_room_size: 5,
                max_room_size: 10,
                max_rooms: 15,
            },
        );

        let map = test_generator.generate();
        map.tiles.iter().enumerate().for_each(|(index, tile)| {
            let position = Vec2::from_index(index as u32, map.size.x);
            SceneSpawner::spawn(
                &mut self.world,
                &self.base_scene,
                owner,
                (Tile::new(*tile), position as Position, Moved, TileChanged),
            );
        });

        self.resources.map = map;

        SceneSpawner::spawn(
            &mut self.world,
            &self.base_scene,
            owner,
            (
                Player,
                Position::new(4, 4),
                Moved,
                Tile::new(TileId::Player),
                TileChanged,
            ),
        );

        self.sync_godot(owner);
    }

    #[export]
    fn update(&mut self, _owner: &Node) {
        self.tick(_owner);
    }

    #[export]
    fn input(&mut self, _owner: &Node, _action: String) {}

    #[export]
    fn move_player(&mut self, owner: &Node, direction: Vector2) {
        events::move_player(&mut self.world, direction.into(), &self.resources.map);
        self.advance_turn(owner);
    }

    fn tick(&mut self, _owner: &Node) {}
    fn advance_turn(&mut self, owner: &Node) {
        self.sync_godot(owner);
    }

    fn sync_godot(&mut self, owner: &Node) {
        sync::update_position(&mut self.world, owner, &self.world_config);
        sync::update_tile(&mut self.world, owner);
    }
}
