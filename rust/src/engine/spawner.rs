use bevy_ecs::world::EntityMut;
use gdnative::prelude::*;

pub struct SceneSpawner;

impl SceneSpawner {
    pub fn spawn(entity: &EntityMut, base_scene: &Ref<PackedScene>, owner: &Node) {
        let spawn = entity.id();
        let scene = unsafe {
            base_scene
                .assume_safe()
                .instance(0)
                .unwrap()
                .assume_unique()
                .cast::<Node2D>()
                .unwrap()
        };

        scene.set_name(spawn.id().to_string());
        owner.add_child(scene, true);
    }
}
