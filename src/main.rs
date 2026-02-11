use bevy::prelude::*;
use rand::prelude::*;

const PLAYER_ROT_SPEED: f32 = 3.5;
const PLAYER_DAMPING: f32 = 0.985;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid;

#[derive(Component)]
struct MoveSpeed(f32);

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct WrapsScreen;

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
        .add_systems(Startup, setup)
        .add_systems(Update, (player_input, apply_velocity, wrap_around))
        .run();
}

fn setup(mut commands: Commands) {
    //camera
    commands.spawn(Camera2d);

    //player
    commands.spawn((
        Player,
        MoveSpeed(300.0),
        Velocity(Vec2::ZERO),
        WrapsScreen,
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.3),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
    ));

    let mut rng = rand::rng();

    //'roids
    for _i in 1..6 {
        let p_x: f32 = rng.random_range(-375.0..375.0);
        let p_y: f32 = rng.random_range(-500.0..500.0);
        let v_x: f32 = rng.random_range(-45.0..45.0);
        let v_y: f32 = rng.random_range(-45.0..45.0);
        commands.spawn((
            Asteroid,
            Velocity(Vec2::new(v_x, v_y)),
            WrapsScreen,
            Sprite {
                color: Color::srgb(0.85, 0.75, 0.25),
                custom_size: Some(Vec2::new(48.0, 48.0)),
                ..default()
            },
            Transform {
                translation: Vec3::new(p_x, p_y, 9.0),
                 ..default()
            }
        ));
    }
}

fn player_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&MoveSpeed, &mut Transform, &mut Velocity), With<Player>>,
) {
    if let Ok((speed, mut transform, mut velocity)) = query.single_mut() {
        let dt = time.delta_secs();

        if keyboard.pressed(KeyCode::KeyA) {
            transform.rotate_z(PLAYER_ROT_SPEED * dt);
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.rotate_z(-PLAYER_ROT_SPEED * dt);
        }
        if keyboard.pressed(KeyCode::KeyW) {
            let forward = (transform.rotation * Vec3::Y).truncate();
            **velocity += forward * speed.0 * dt;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            let backward = (transform.rotation * Vec3::Y).truncate();
            **velocity -= backward * (speed.0 * 0.5) * dt;
        }

        **velocity *= PLAYER_DAMPING.powf(dt * 60.0);
    }
}

fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += Vec3::new(velocity.x, velocity.y, 0.0) * time.delta_secs();
    }
}

fn wrap_axis(value: &mut f32, half: f32) {
    if value.abs() > half {
        *value = -value.signum() * half;
    }
}

fn wrap_around(mut query: Query<&mut Transform, With<WrapsScreen>>) {
    let half_width = 750.0 / 2.0;
    let half_height = 1000.0 / 2.0;

    for mut transform in &mut query {
        wrap_axis(&mut transform.translation.x, half_width);
        wrap_axis(&mut transform.translation.y, half_height);
    }
}