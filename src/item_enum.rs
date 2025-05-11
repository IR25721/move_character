use bevy::prelude::*;

#[derive(Component, PartialEq, Eq, Debug, Hash, Clone, Copy)]
pub enum Items {
    Potion,
    Ether,
    TeleportStone,
    Antidote,
    Torch,
}
