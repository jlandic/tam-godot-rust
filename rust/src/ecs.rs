use bevy_ecs::prelude::{Schedule, Stage, SystemStage, World};
use gdnative::api::*;
use gdnative::prelude::*;

use crate::config::WorldConfig;
use crate::data::geo::{Vec2, Vec2Unsigned};
use crate::data::tiles::TileId;
use crate::engine::{resources::MovementInput, SceneSpawner};
use crate::generators::{TestGenerator, TestGeneratorRoomConfig};

use self::components::{Player, Tile, Transform};

mod components;
mod sync;
mod systems;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Ecs {
    world: World,
    schedule: Schedule,
    #[property]
    base_scene: Ref<PackedScene>,
}

#[methods]
impl Ecs {
    fn new(_owner: &Node) -> Self {
        Self {
            world: World::new(),
            schedule: Schedule::default(),
            base_scene: PackedScene::new().into_shared(),
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
                self.world
                    .spawn()
                    .insert(Tile::new(*tile))
                    .insert(Transform::new(position)),
                &self.base_scene,
                owner,
            );
        });

        SceneSpawner::spawn(
            self.world
                .spawn()
                .insert(Player {})
                .insert(Transform::new(Vec2::new(4, 4)))
                .insert(Tile::new(TileId::Player)),
            &self.base_scene,
            owner,
        );

        self.world.insert_resource(map);
        self.world.insert_resource(WorldConfig::default());
        self.world.insert_resource(MovementInput::default());

        self.schedule.add_stage(
            "inputs",
            SystemStage::parallel().with_system(systems::move_player),
        );

        self.sync_godot(owner)
    }

    #[export]
    fn update(&mut self, _owner: &Node) {
        self.tick(_owner);
    }

    #[export]
    fn input(&mut self, _owner: &Node, _action: String) {}

    #[export]
    fn move_player(&mut self, owner: &Node, direction: Vector2) {
        // TODO: find if events can be sent via World instance or not
        // events::move_player(&mut self.world, direction.into());
        let mut movement = self.world.get_resource_mut::<MovementInput>().unwrap();
        movement.direction = direction.into();

        self.advance_turn(owner);
    }

    fn tick(&mut self, _owner: &Node) {}
    fn advance_turn(&mut self, owner: &Node) {
        self.schedule.run(&mut self.world);
        self.sync_godot(owner);
    }

    fn sync_godot(&mut self, owner: &Node) {
        sync::update_position(&mut self.world, owner);
        sync::update_tile(&mut self.world, owner);
    }
}
