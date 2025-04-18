use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let world_handle: Handle<TiledMap> = asset_server.load("seconmap.tmx");
    commands.spawn((TiledMapHandle(world_handle), TiledMapAnchor::Center));
}
