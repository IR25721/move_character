use bevy::{color::palettes::css::*, prelude::*};
#[derive(Resource)]
pub struct SelectedButton(pub ButtonId);
impl Default for SelectedButton {
    fn default() -> Self {
        SelectedButton(ButtonId::Items)
    }
}
const BUTTON_ORDER: [ButtonId; 3] = [ButtonId::Items, ButtonId::Status, ButtonId::Setting];

#[derive(Component)]
pub struct MenuButton;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum ButtonId {
    Items,
    Status,
    Setting,
}

#[derive(Resource)]
pub struct ButtonAnimation {
    pub is_open: bool,
    pub target_y: f32,
    pub current_y: f32,
    pub velocity: f32,
}

impl Default for ButtonAnimation {
    fn default() -> Self {
        Self {
            is_open: false,
            target_y: 1100.0,
            current_y: 800.0,
            velocity: 50.0,
        }
    }
}
pub fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            MenuButton,
            ButtonId::Items,
            Node {
                display: Display::Grid,
                position_type: PositionType::Absolute,
                left: Val::Px(100.),
                width: Val::Px(300.),
                height: Val::Px(100.),
                overflow: Overflow::clip(),
                ..default()
            },
            BorderRadius::all(Val::Percent(25.)),
            BackgroundColor(WHITE.into()),
            Outline {
                width: Val::Px(5.0),
                offset: Val::Px(0.0),
                color: WHITE.into(),
            },
        ))
        .with_children(|p| {
            p.spawn((
                ImageNode::new(assets.load("book.png")),
                Node {
                    top: Val::Px(10.),
                    left: Val::Px(95.),
                    width: Val::Px(120.),
                    height: Val::Px(80.),
                    ..default()
                },
            ));
        });
    commands
        .spawn((
            MenuButton,
            ButtonId::Status,
            Node {
                display: Display::Grid,
                left: Val::Px(450.),
                width: Val::Px(300.),
                height: Val::Px(100.),
                overflow: Overflow::clip(),

                ..default()
            },
            BorderRadius::all(Val::Percent(25.)),
            BackgroundColor(WHITE.into()),
            Outline {
                width: Val::Px(5.0),
                offset: Val::Px(0.0),
                color: WHITE.into(),
            },
        ))
        .with_children(|p| {
            p.spawn((
                ImageNode::new(assets.load("team.png")),
                Node {
                    top: Val::Px(10.),
                    left: Val::Px(95.),
                    width: Val::Px(120.),
                    height: Val::Px(80.),
                    ..default()
                },
            ));
        });
    commands
        .spawn((
            MenuButton,
            ButtonId::Setting,
            Node {
                display: Display::Grid,
                top: Val::Px(800.),
                left: Val::Px(800.),
                width: Val::Px(300.),
                height: Val::Px(100.),
                overflow: Overflow::clip(),

                ..default()
            },
            BorderRadius::all(Val::Percent(25.)),
            BackgroundColor(WHITE.into()),
            Outline {
                width: Val::Px(5.0),
                offset: Val::Px(0.0),
                color: WHITE.into(),
            },
        ))
        .with_children(|p| {
            p.spawn((
                ImageNode::new(assets.load("setting.png")),
                Node {
                    top: Val::Px(10.),
                    left: Val::Px(95.),
                    width: Val::Px(120.),
                    height: Val::Px(80.),
                    ..default()
                },
            ));
        });
}

pub fn moveup_button_input(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut animation: ResMut<ButtonAnimation>,
) {
    if kb_input.just_pressed(KeyCode::KeyG) {
        animation.is_open = !animation.is_open;
        animation.target_y = if animation.is_open { 800.0 } else { 1100.0 };
    }
}

pub fn animate_button_position(
    mut query: Query<&mut Node, With<MenuButton>>,
    mut animation: ResMut<ButtonAnimation>,
) {
    let target = animation.target_y;
    let v = animation.velocity;
    let mut y = animation.current_y;

    if (y - target).abs() > 1.0 {
        if y < target {
            y += v;
            if y > target {
                y = target;
            }
        } else {
            y -= v;
            if y < target {
                y = target;
            }
        }

        animation.current_y = y;
    }

    for mut style in query.iter_mut() {
        style.top = Val::Px(y);
    }
}
pub fn setup_selected_button(mut commands: Commands) {
    commands.insert_resource(SelectedButton(ButtonId::Items));
}

pub fn update_selected_button(
    kb_input: Res<ButtonInput<KeyCode>>,
    mut selected: ResMut<SelectedButton>,
    animation: ResMut<ButtonAnimation>,
) {
    if animation.is_open {
        let current = selected.0;
        let idx = BUTTON_ORDER.iter().position(|&b| b == current).unwrap();

        if kb_input.just_pressed(KeyCode::ArrowRight) {
            let next = (idx + 1) % BUTTON_ORDER.len();
            selected.0 = BUTTON_ORDER[next];
        } else if kb_input.just_pressed(KeyCode::ArrowLeft) {
            let prev = (idx + BUTTON_ORDER.len() - 1) % BUTTON_ORDER.len();
            selected.0 = BUTTON_ORDER[prev];
        }
    }
}

pub fn trigger_button_action(
    kb_input: Res<ButtonInput<KeyCode>>,
    selected: Res<SelectedButton>,
    animation: ResMut<ButtonAnimation>,
) {
    if kb_input.just_pressed(KeyCode::KeyF) && animation.is_open {
        println!("Selected button: {:?}", selected.0);
    }
}
pub fn update_button_outline(
    selected: Res<SelectedButton>,
    mut query: Query<(&ButtonId, &mut Outline), With<MenuButton>>,
) {
    for (id, mut outline) in query.iter_mut() {
        if *id == selected.0 {
            outline.color = Color::BLACK;
        } else {
            outline.color = Color::NONE;
        }
    }
}
