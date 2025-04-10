use bevy::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle {
        camera: Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(256, 256),
                ..default()
            }),
            ..default()
        },
        ..default()
    },));
}
