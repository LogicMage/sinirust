use bevy::prelude::*;

pub const PLAYER_ROT_SPEED: f32 = 3.5;
pub const PLAYER_DAMPING: f32 = 0.985;

//the camera sees 750x1000
//the world is 4000x4000, meaning there is roughly 3000 pixels of off-screen space that you have to traverse before you see an object loop around
pub const WORLD_WIDTH: f32 = 4000.0;
pub const WORLD_HEIGHT: f32 = 4000.0;

#[derive(Resource, Default)]
pub struct GameScore(pub u32);

#[derive(Resource, Default)]
pub struct Sinibombs(pub u32);