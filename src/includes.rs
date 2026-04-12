use bevy::prelude::*;

//the camera sees 750x1000
//the world is 4000x4000, meaning there is roughly 3000 pixels of off-screen space that you have to traverse before you see an object loop around
pub const WORLD_WIDTH: f32 = 4000.0;
pub const WORLD_HEIGHT: f32 = 4000.0;

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct GameScore(pub u32, pub u32);