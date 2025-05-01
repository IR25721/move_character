use crate::character::Player;
use bevy::prelude::*;
#[derive(Component)]
pub struct CameraMarker;
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        CameraMarker,
        IsDefaultUiCamera,
        Camera2d,
    ));
}
pub fn move_camera_with_player(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    mut player: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(player) = player.get_single_mut() else {
        return;
    };

    let Vec3 { x, y, .. } = player.translation;
    let direction = Vec3::new(x, y, camera.translation.z);
    camera
        .translation
        .smooth_nudge(&direction, 2., time.delta_secs());
}
