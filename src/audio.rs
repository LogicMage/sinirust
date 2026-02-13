use bevy::prelude::*;
use bevy::audio::*;

#[derive(Component)]
pub struct Music;

pub fn spawn_music(commands: &mut Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("audio/music.ogg")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(0.5),
            ..default()
        },
        Music,
    ));
}
