use gdnative::{
    api::{TileMap, TileSet},
    prelude::*,
};

pub unsafe fn get_tile_map(
    tile_map_name: &str,
    owner: &Node,
    recursive: bool,
) -> Option<Ref<TileMap, Unique>> {
    owner
        .find_node(tile_map_name, recursive, false)
        .unwrap_or_else(|| panic!("No child found with name {}", tile_map_name))
        .assume_unique()
        .cast::<TileMap>()
}

pub unsafe fn get_tile_set(
    tile_map_name: &str,
    owner: &Node,
    recursive: bool,
) -> Option<Ref<TileSet, Unique>> {
    get_tile_map(tile_map_name, owner, recursive)
        .expect("Expected child to be type TileMap")
        .tileset()
        .map(|tile_set| tile_set.assume_unique())
}

pub unsafe fn get_entity_node(owner: &Node, id: &str) -> Option<Ref<Node2D, Unique>> {
    owner
        .find_node(&id, true, false)
        .unwrap_or_else(|| panic!("No entity found with ID {}", &id))
        .assume_unique()
        .cast::<Node2D>()
}
