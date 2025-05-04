use bevy::prelude::*;

#[derive(Component)]
pub struct FlowingText {
    pub content: String,
    pub index: usize,
    pub timer: Timer,
}

pub fn flowing_text(mut query: Query<(&mut Text, &mut FlowingText)>, time: Res<Time>) {
    for (mut text, mut flowing) in query.iter_mut() {
        flowing.timer.tick(time.delta());

        if flowing.timer.finished() {
            let total_chars = flowing.content.chars().count();

            if flowing.index < total_chars {
                flowing.index += 1;

                let displayed: String = flowing.content.chars().take(flowing.index).collect();
                *text = Text::new(displayed);
            }
        }
    }
}
