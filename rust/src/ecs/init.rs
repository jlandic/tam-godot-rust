use bevy_ecs::event::Events;
use bevy_ecs::prelude::World;
use bevy_ecs::schedule::{Schedule, SystemStage};
use gdnative::prelude::*;

use crate::config::WorldConfig;
use crate::data::geo::{Map, Vec2};
use crate::data::tiles::TileId;
use crate::ecs::components::{MapTile, Player, Tile, Transform};
use crate::ecs::factories::TestGeneratorFactory;
use crate::ecs::{event_manager, sync, systems};
use crate::engine::events::{MapTileUpdated, MovementInput};
use crate::engine::resources::TileSetIdMapper;
use crate::engine::shared::TILE_MAP_COMPONENT_NAME;
use crate::engine::tree;
use crate::engine::SceneSpawner;

use super::components::Viewshed;

pub fn setup_resources(owner: &Node, world: &mut World, base_scene: &Ref<PackedScene>) {
    world.insert_resource(Events::<MovementInput>::default());
    world.insert_resource(Events::<MapTileUpdated>::default());
    world.insert_resource(WorldConfig::default());

    let map = initialize_map(world);
    let tile_set_id_mapper = initialize_tile_set_id_mapper(owner);
    initialize_player(world, owner, base_scene, &map);

    world.insert_resource(map);
    world.insert_resource(tile_set_id_mapper);
    world.insert_resource(Events::<MovementInput>::default());
    world.insert_resource(Events::<MapTileUpdated>::default());
    world.insert_resource(WorldConfig::default());
    world.insert_resource(unsafe { owner.assume_shared() });
}

pub fn setup_schedule(schedule: &mut Schedule) {
    schedule.add_stage(
        "inputs",
        SystemStage::parallel()
            .with_system(systems::move_player),
    );
    schedule.add_stage(
        "update_world",
        SystemStage::parallel()
            .with_system(systems::calculate_viewshed),
    );
    schedule.add_stage(
        "update_godot",
        SystemStage::single_threaded()
            .with_system(sync::update_map)
            .with_system(sync::update_tile)
            .with_system(sync::update_position)
            .with_system(sync::update_viewshed),
    );
    schedule.add_stage(
        "purge_events",
        SystemStage::parallel().with_system(event_manager::purge_events::<MovementInput>),
    );
}

fn initialize_map(world: &mut World) -> Map {
    let mut updates: Vec<MapTileUpdated> = Vec::new();

    let mut test_generator = TestGeneratorFactory::default();
    let mut map = test_generator.generate();
    let ids = map
        .tiles
        .iter()
        .enumerate()
        .map(|(i, tile)| {
            let position = Vec2::from_index(i as u32, map.size.x);
            updates.push(MapTileUpdated {
                position,
                tile: *tile,
            });

            Some(
                world
                    .spawn()
                    .insert(Transform::new(position))
                    .insert(Tile::new(*tile))
                    .insert(MapTile {})
                    .id(),
            )
        })
        .map(|entity| entity.unwrap().id())
        .collect::<Vec<u32>>();
    map.tile_entities = ids;

    let mut events = world.get_resource_mut::<Events<MapTileUpdated>>().unwrap();
    for update in updates {
        events.send(update);
    }

    map
}

fn initialize_tile_set_id_mapper(owner: &Node) -> TileSetIdMapper {
    let tile_set = unsafe {
        tree::get_tile_set(TILE_MAP_COMPONENT_NAME, owner, false)
            .expect("TileMap does not contain any tileset")
    };

    TileSetIdMapper::new(tile_set)
}

fn initialize_player(world: &mut World, owner: &Node, base_scene: &Ref<PackedScene>, map: &Map) {
    let first_free_position = map.first_free_position().expect("Map has no free tile");
    SceneSpawner::spawn(
        world
            .spawn()
            .insert(Player {})
            .insert(Transform::new(first_free_position))
            .insert(Tile::new(TileId::Player))
            .insert(Viewshed::new(10)),
        base_scene,
        owner,
    );
}
