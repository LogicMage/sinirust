use bevy::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    None,
    Player,
    Enemy
}