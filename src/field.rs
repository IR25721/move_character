use crate::tiled;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map_handle = tiled::TiledMapHandle(asset_server.load("first.tmx"));
    commands.spawn(tiled::TiledMapBundle {
        tiled_map: map_handle,
        ..Default::default()
    });
}
