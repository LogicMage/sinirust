use bevy::prelude::*;

use crate::{includes::*};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct ExitButton;

#[derive(Component)]
struct MenuButton;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    InGame,
}

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<GameScore>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn((Camera2d, MainCamera));

    //root container
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::BLACK),
        MenuUI,
    ))
    .with_children(|parent| {
        //title
        parent.spawn((
            Text::new("SINIRUST"),
            TextFont {
                font: font.clone(),
                font_size: 64.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        //highscore
        parent.spawn((
            Text::new(format!("High Score: {}", score.1)),
            TextFont {
                font: font.clone(),
                font_size: 32.0,
                ..default()
            },
            TextColor(Color::WHITE),
        ));

        //play button
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            Button,
            MenuButton,
            PlayButton,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Play"),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(HOVERED_BUTTON),
            ));
        });

        //exit button
        parent.spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON),
            Button,
            MenuButton,
            ExitButton,
        ))
        .with_children(|p| {
            p.spawn((
                Text::new("Exit"),
                TextFont {
                    font: font.clone(),
                    font_size: 30.0,
                    ..default()
                },
                TextColor(HOVERED_BUTTON),
            ));
        });
    });
}

pub fn menu_interactions(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            Option<&PlayButton>,
            Option<&ExitButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, mut color, play, exit_btn) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::BLACK);

                if play.is_some() {
                    next_state.set(GameState::InGame);
                }

                if exit_btn.is_some() {
                    exit.write(AppExit::Success);
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::WHITE);
            }
            Interaction::None => {
                *color = if play.is_some() {
                    BackgroundColor(Color::BLACK)
                } else {
                    BackgroundColor(Color::BLACK)
                };
            }
        }
    }
}

pub fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}