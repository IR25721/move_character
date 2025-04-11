use bevy::prelude::*;
const PLAYER_SPEED: f32 = 5.;
#[derive(Component)]
pub struct Player;
pub fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("woman_walking.png");
    let texture_atlas_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 24), 6, 4, None, None);
    let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
    commands.spawn((Player,Sprite::from_atlas_image(
        texture_handle,
        TextureAtlas {
            layout: texture_atlas_layout_handle,
            index: 0,
        },
    )));
}
pub fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };
        let mut direction = Vec2::ZERO;

        if kb_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }
        if kb_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if kb_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }
        if kb_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }
    let move_delta = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    player.translation += move_delta.extend(0.);
}
