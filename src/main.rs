mod animation;
mod camera;
mod character;
use animation::*;
use bevy::prelude::*;
use camera::*;
use character::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (setup_character, setup_camera, set_field))
        .add_event::<Walking>()
        .add_systems(
            Update,
            (move_player, move_camera_with_player, animate_sprite).chain(),
        )
        .run();
}

fn set_field(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000., 700.))),
        MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.3))),
    ));
}
