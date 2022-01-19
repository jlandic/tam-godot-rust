use gdnative::{
    api::{TileMap, TileSet},
    prelude::*,
};

pub unsafe fn get_tile_map(tile_map_name: &str, owner: &Node) -> Option<Ref<TileMap, Unique>> {
    owner
        .find_node(tile_map_name, false, false)
        .unwrap_or_else(|| panic!("No child found with name {}", tile_map_name))
        .assume_unique()
        .cast::<TileMap>()
}

pub unsafe fn get_tile_set(tile_map_name: &str, owner: &Node) -> Option<Ref<TileSet, Unique>> {
    get_tile_map(tile_map_name, owner)
        .expect("Expected child to be type TileMap")
        .tileset()
        .map(|tile_set| tile_set.assume_unique())
}
