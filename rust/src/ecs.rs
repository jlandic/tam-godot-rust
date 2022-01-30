use bevy_ecs::event::Events;
use bevy_ecs::prelude::{Schedule, Stage, World};
use gdnative::api::*;
use gdnative::prelude::*;

use crate::engine::events::{DebugAction, DebugConfigUpdate, MovementInput};

mod components;
mod event_manager;
mod factories;
mod init;
mod resources;
mod sync;
mod systems;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Ecs {
    world: World,
    schedule: Schedule,
    // non-diegetic schedule (does not update the in-game world)
    meta_schedule: Schedule,
    #[property]
    base_scene: Ref<PackedScene>,
}

#[methods]
impl Ecs {
    fn new(_owner: &Node) -> Self {
        Self {
            world: World::new(),
            schedule: Schedule::default(),
            meta_schedule: Schedule::default(),
            base_scene: PackedScene::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        init::setup_resources(owner, &mut self.world, &self.base_scene);
        init::setup_schedule(&mut self.schedule);
        init::setup_meta_schedule(&mut self.meta_schedule);

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

    #[export]
    fn toggle_draw_path(&mut self, _owner: &Node) {
        self.send_debug_config_update_event(DebugAction::ToggleDrawPath)
    }
    #[export]
    fn toggle_no_clip(&mut self, _owner: &Node) {
        self.send_debug_config_update_event(DebugAction::ToggleNoClip)
    }

    fn execute_turn(&mut self) {
        self.schedule.run(&mut self.world);
        self.meta_schedule.run(&mut self.world);
    }

    fn update_without_turn(&mut self) {
        self.meta_schedule.run(&mut self.world);
    }

    fn send_debug_config_update_event(&mut self, action: DebugAction) {
        let mut events = self
            .world
            .get_resource_mut::<Events<DebugConfigUpdate>>()
            .unwrap();
        events.send(DebugConfigUpdate { action });

        self.update_without_turn();
    }
}
