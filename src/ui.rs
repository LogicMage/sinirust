use bevy::prelude::*;

use crate::includes::*;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct SinibombsText;

pub fn setup_score_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Text::new("Score: 0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        ScoreText,
    ));
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            left: Val::Px(10.0),
            ..default()
        },
        Text::new("Sinibombs: 0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::WHITE),
        SinibombsText,
    ));
}

pub fn update_score_text(
    score: Res<GameScore>,
    sini: Res<Sinibombs>,
    mut queries: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<SinibombsText>>,
    )>,
) {
    if score.is_changed() {
        for mut text in &mut queries.p0() {
            text.0 = format!("Score: {}", score.0);
        }
    }
    if sini.is_changed() {
        for mut text in &mut queries.p1() {
            text.0 = format!("Sinibombs: {}", sini.0);
        }
    }
}