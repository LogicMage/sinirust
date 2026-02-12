mod physics;
mod shooting;

use bevy::math::*;
use bevy::prelude::*;
use physics::*;
use rand::prelude::*;
use shooting::*;

const PLAYER_ROT_SPEED: f32 = 3.5;
const PLAYER_DAMPING: f32 = 0.985;

//the camera sees 750x1000
//the world is 4000x4000, meaning there is roughly 3000 pixels of off-screen space that you have to traverse before you see an object loop around
const WORLD_WIDTH: f32 = 4000.0;
const WORLD_HEIGHT: f32 = 4000.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Asteroid;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct MoveSpeed(f32);

#[derive(Component)]
struct WrapsAroundCamera;

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
    .add_systems(
        Update,
        (
            player_movement_input,
            apply_velocity,
            handle_collisions,
            player_shooting_input,
            gun_system,
            projectile_system,
            camera_follow,
            wrap_around_camera,
        )
        .chain(),
    )
    .add_message::<ShootMessage>()
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn((Camera2d, MainCamera));

    commands.spawn((
        Player,
        MoveSpeed(500.0),
        Velocity(Vec2::ZERO),
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.3),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        Collider { radius: 15.0 },
        Mass(10.0),
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.0, 0.0, 1.0)))),
        Gun {
            cooldown: 0.5,
            timer: 0.0,
            projectile_speed: 1000.0,
        }
    ));

    let mut rng = rand::rng();

    //asteroid spawner
    for _i in 0..100 {
        let half_w = WORLD_WIDTH / 2.0;
        let half_h = WORLD_HEIGHT / 2.0;
        
        let p_x: f32 = rng.random_range(-half_w..half_w);
        let p_y: f32 = rng.random_range(-half_h..half_h);
        
        let v_x: f32 = rng.random_range(-20.0..20.0);
        let v_y: f32 = rng.random_range(-20.0..20.0);
        
        commands.spawn((
            Asteroid,
            Velocity(Vec2::new(v_x, v_y)),
            WrapsAroundCamera,
            Transform {
                translation: Vec3::new(p_x, p_y, 9.0),
                ..default()
            },
            Collider { radius: 30.0 },
            Mass(50.0),
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
        ));
    }
}

fn player_movement_input(keyboard: Res<ButtonInput<KeyCode>>, time: Res<Time>,
    mut query: Query<(&MoveSpeed, &mut Transform, &mut Velocity), With<Player>>,) 
{
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

fn camera_follow(player_query: Query<&Transform, With<Player>>, mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,)
{
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}

fn wrap_around_camera(camera_query: Query<&Transform, With<MainCamera>>, mut object_query: Query<&mut Transform,
    (With<WrapsAroundCamera>, Without<MainCamera>)>,) 
{
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