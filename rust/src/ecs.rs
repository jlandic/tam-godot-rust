use bevy_ecs::event::Events;
use bevy_ecs::prelude::{Schedule, Stage, World};
use gdnative::api::*;
use gdnative::prelude::*;

mod components;
mod event_manager;
mod factories;
mod init;
mod resources;
mod sync;
mod systems;

use crate::engine::events::MovementInput;

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
        init::setup_resources(owner, &mut self.world, &self.base_scene);
        init::setup_schedule(&mut self.schedule);

        self.execute_turn();
    }

    #[export]
    fn update(&mut self, _owner: &Node) {}

    #[export]
    fn input(&mut self, _owner: &Node, _action: String) {}

    #[export]
    fn move_player(&mut self, _owner: &Node, direction: Vector2) {
        let mut events = self
            .world
            .get_resource_mut::<Events<MovementInput>>()
            .unwrap();
        events.send(MovementInput::from(direction));

        self.execute_turn();
    }

    fn execute_turn(&mut self) {
        self.schedule.run(&mut self.world);
    }
}
