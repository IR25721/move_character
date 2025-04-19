use crate::character::Player;
use crate::character::Walking;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Sprite), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut is_reverse: Local<bool>,
    mut reader: EventReader<Walking>,
) {
    let mut _first: usize = 0;
    let mut _last: usize = 0;
    for event in reader.read() {
        _first = event.first;
        _last = event.last;
        if _first == _last {
            return;
        }
        let just_pressed = keyboard_input.just_pressed(KeyCode::KeyW)
            || keyboard_input.just_pressed(KeyCode::KeyA)
            || keyboard_input.just_pressed(KeyCode::KeyS)
            || keyboard_input.just_pressed(KeyCode::KeyD);

        let is_key_pressed = keyboard_input.pressed(KeyCode::KeyW)
            || keyboard_input.pressed(KeyCode::KeyA)
            || keyboard_input.pressed(KeyCode::KeyS)
            || keyboard_input.pressed(KeyCode::KeyD);
        for (mut timer, mut sprite) in &mut query {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if just_pressed {
                    atlas.index = _first;
                    *is_reverse = false;
                    println!("キーが押されたので atlas.index をリセット: {}", atlas.index);
                }

                timer.tick(time.delta());

                if is_key_pressed && timer.just_finished() {
                    if !*is_reverse {
                        atlas.index += 1;
                        println!("Current sprite index: {}", atlas.index);
                        if atlas.index == _last {
                            *is_reverse = true;
                        }
                    } else {
                        atlas.index -= 1;
                        println!("Current sprite index: {}", atlas.index);
                        if atlas.index == _first {
                            *is_reverse = false;
                        }
                    }
                }
            }
        }
    }
}
