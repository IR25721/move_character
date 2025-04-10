use bevy::prelude::*;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d,));
}
