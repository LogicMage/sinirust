mod asteroid;
mod audio;
mod crystal;
mod health;
mod includes;
mod menu;
mod navigation;
mod physics;
mod player;
mod shooting;
mod sinibomb;
mod sinistar;
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
use menu::*;
use physics::*;
use player::*;
use shooting::*;
use sinibomb::*;
use sinistar::*;
use ui::*;
use warrior::*;
use worker::*;

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
        IntoScheduleConfigs::into_configs(menu_interactions).run_if(in_state(GameState::Menu)),
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
            spawn_sinistar,
            setup_score_ui,
        )
            .chain(),
    )
    .add_systems(
        Update,
        (
            player_movement_input,
            player_weapon_input,
            (
                worker_roaming_ai,
                worker_sensor_ai,
                worker_returning_ai,
                worker_return_deposit,
                worker_movement,
            )
                .chain(),
            (warrior_ai, warrior_movement).chain(),
            sinistar_chase,
            crystal_impacts,
            gun_system,
            projectile_system,
            launcher_system,
            bomb_system,
            apply_velocity,
            handle_collisions,
            update_score_text,
            camera_follow,
            wrap_around_camera,
        )
            .chain()
            .run_if(in_state(GameState::InGame)),
    )
    .add_message::<ShootMessage>()
    .add_message::<LaunchMessage>()
    .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    sounds: Res<AudioAssets>,
) {
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

        if keyboard.pressed(KeyCode::ArrowLeft) {
            transform.rotate_z(PLAYER_ROT_SPEED * dt);
        }
        if keyboard.pressed(KeyCode::ArrowRight) {
            transform.rotate_z(-PLAYER_ROT_SPEED * dt);
        }
        if keyboard.pressed(KeyCode::ArrowUp) {
            let forward = (transform.rotation * Vec3::Y).truncate();
            **velocity += forward * player.speed * dt;
        }
        if keyboard.pressed(KeyCode::ArrowDown) {
            let backward = (transform.rotation * Vec3::Y).truncate();
            **velocity -= backward * (player.speed * 0.5) * dt;
        }

        **velocity *= PLAYER_DAMPING.powf(dt * 90.0);
    }
}

fn player_weapon_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Player>>,
    mut shoot_writer: MessageWriter<ShootMessage>,
    mut launch_writer: MessageWriter<LaunchMessage>,
) {
    if keyboard.pressed(KeyCode::KeyZ) {
        if let Ok(entity) = query.single() {
            shoot_writer.write(ShootMessage { entity });
        }
    }

    if keyboard.pressed(KeyCode::KeyX) {
        if let Ok(entity) = query.single() {
            launch_writer.write(LaunchMessage { entity });
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
