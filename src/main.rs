mod animation;
mod camera;
mod character;
mod field;
use animation::*;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use camera::*;
use character::*;
use field::*;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Tiled Map Editor Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(100.),
        ))
        .add_plugins(TiledMapPlugin::default())
        .add_plugins(TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default())
        .add_systems(Startup, (setup_character, setup_camera, startup))
        .add_event::<Walking>()
        .add_systems(
            Update,
            (move_player, move_camera_with_player, animate_sprite).chain(),
        )
        .run();
}
