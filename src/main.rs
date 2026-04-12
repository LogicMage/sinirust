mod asteroid;
mod audio;
mod crystal;
mod health;
mod includes;
mod navigation;
mod physics;
mod player;
mod shooting;
mod team;
mod ui;
mod warrior;
mod worker;

use asteroid::*;
use audio::*;
use bevy::math::*;
use bevy::prelude::*;
use crystal::*;
use includes::*;
use physics::*;
use player::*;
use shooting::*;
use ui::*;
use warrior::*;
use worker::*;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct MenuUI;

#[derive(Component)]
struct PlayButton;

#[derive(Component)]
struct ExitButton;

#[derive(Component)]
struct MenuButton;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Menu,
    InGame,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<GameScore>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn(Camera2d);

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

fn menu_interactions(
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

fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Sinirust".into(),
            resolution: (750, 1000).into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .init_state::<GameState>()
    .init_resource::<GameScore>()
    .init_resource::<Sinibombs>()

    //menu
    .add_systems(OnEnter(GameState::Menu), setup_menu)
    .add_systems(
    Update,
    IntoScheduleConfigs::into_configs(menu_interactions)
        .run_if(in_state(GameState::Menu)),
        
    )
    .add_systems(OnExit(GameState::Menu), cleanup_menu)

    // GAME
    .add_systems(
        OnEnter(GameState::InGame),
        (
            load_sounds,
            setup,
            spawn_asteroids,
            spawn_workers,
            spawn_warriors,
            setup_score_ui,
        )
            .chain(),
    )
    .add_systems(
        Update,
        (
            player_movement_input,
            worker_roaming_ai,
            worker_sensor_ai,
            worker_movement,
            warrior_ai,
            warrior_movement,
            apply_velocity,
            handle_collisions,
            crystal_impacts,
            player_shooting_input,
            gun_system,
            projectile_system,
            update_score_text,
            camera_follow,
            wrap_around_camera,
        )
            .chain()
            .run_if(in_state(GameState::InGame)),
    )
    .add_message::<ShootMessage>()
    .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    sounds: Res<AudioAssets>,
) {
    commands.spawn((Camera2d, MainCamera));

    spawn_player(&mut commands, meshes, materials);

    spawn_music(&mut commands, sounds.music.clone());
}

fn player_movement_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Player, &mut Velocity)>,
) {
    if let Ok((mut transform, player, mut velocity)) = query.single_mut() {
        let dt = time.delta_secs();

        if keyboard.pressed(KeyCode::KeyA) {
            transform.rotate_z(PLAYER_ROT_SPEED * dt);
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.rotate_z(-PLAYER_ROT_SPEED * dt);
        }
        if keyboard.pressed(KeyCode::KeyW) {
            let forward = (transform.rotation * Vec3::Y).truncate();
            **velocity += forward * player.speed * dt;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            let backward = (transform.rotation * Vec3::Y).truncate();
            **velocity -= backward * (player.speed * 0.5) * dt;
        }

        **velocity *= PLAYER_DAMPING.powf(dt * 90.0);
    }
}

fn player_shooting_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Player>>,
    mut writer: MessageWriter<ShootMessage>,
) {
    if keyboard.pressed(KeyCode::Space) {
        if let Ok(entity) = query.single() {
            writer.write(ShootMessage { entity });
        }
    }
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn wrap_around_camera(
    camera_query: Query<&Transform, With<MainCamera>>,
    mut object_query: Query<&mut Transform, (With<WrapsAroundCamera>, Without<MainCamera>)>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };
    let cam_pos = camera_transform.translation.truncate();

    let half_width = WORLD_WIDTH / 2.0;
    let half_height = WORLD_HEIGHT / 2.0;

    for mut obj_transform in &mut object_query {
        let obj_pos = obj_transform.translation.truncate();
        let diff = obj_pos - cam_pos;

        if diff.x > half_width {
            obj_transform.translation.x -= WORLD_WIDTH;
        } else if diff.x < -half_width {
            obj_transform.translation.x += WORLD_WIDTH;
        }

        if diff.y > half_height {
            obj_transform.translation.y -= WORLD_HEIGHT;
        } else if diff.y < -half_height {
            obj_transform.translation.y += WORLD_HEIGHT;
        }
    }
}
