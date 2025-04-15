mod animation;
mod camera;
mod character;
mod field;
mod tiled;
use animation::*;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use camera::*;
use character::*;
use field::*;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tiled Map Editor Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_plugins(tiled::TiledMapPlugin)
        .add_systems(Startup, (setup_character, setup_camera, startup))
        .add_event::<Walking>()
        .add_systems(
            Update,
            (move_player, move_camera_with_player, animate_sprite).chain(),
        )
        .run();
}
