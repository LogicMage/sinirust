//use bevy::asset;
use bevy::prelude::*;
use bevy::audio::*;

#[derive(Resource)]
pub struct AudioAssets {
    pub music: Handle<AudioSource>,
    pub shoot: Handle<AudioSource>,
    pub launch: Handle<AudioSource>,
    pub explode: Handle<AudioSource>,
    pub beware: Handle<AudioSource>,
}

pub fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(AudioAssets {
        music: asset_server.load("audio/music.ogg"),
        shoot: asset_server.load("audio/shoot.ogg"),
        launch: asset_server.load("audio/launch.ogg"),
        explode: asset_server.load("audio/explode.ogg"),
        beware: asset_server.load("audio/beware.ogg"),
    });
}

#[derive(Component)]
pub struct Music;

pub fn spawn_music(commands: &mut Commands, music: Handle<AudioSource>) {
    commands.spawn((
        AudioPlayer::new(music),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.5),
            ..default()
        },
        Music,
    ));
}