use crate::{health::*, navigation::*, physics::*, player::*, shooting::*, team::*};
use bevy::prelude::*;
use rand::prelude::*;

const WARRIOR_RADIUS: f32 = 12.0;

#[derive(Component)]
pub struct Warrior {
    pub acceleration: f32,
    pub detection_radius: f32,
}

pub fn spawn_warriors(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();

    for _ in 0..5 {
        let p_x = rng.random_range(-1000.0..1000.0);
        let p_y = rng.random_range(-1000.0..1000.0);

        commands.spawn((
            Transform::from_xyz(p_x, p_y, 0.0),
            Warrior {
                acceleration: 200.0,
                detection_radius: 500.0,
            },
            Velocity(Vec2::ZERO),
            Collider {
                radius: WARRIOR_RADIUS,
            },
            Mass(6.0),
            Mesh2d(meshes.add(RegularPolygon::new(WARRIOR_RADIUS, 5))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 0.0)))),
            Gun {
                cooldown: 3.0,
                timer: 0.0,
                projectile_speed: 500.0,
            },
            Health(1),
            Team::Enemy,
        ));
    }
}

pub fn warrior_ai(
    mut commands: Commands,
    mut warriors: Query<(Entity, &Transform, &Warrior, Option<&NavigationTarget>)>,
    players: Query<(Entity, &Transform), With<Player>>,
    mut writer: MessageWriter<ShootMessage>,
) {
    let mut rng = rand::rng();

    for (warrior_entity, warrior_transform, warrior, target) in &mut warriors {
        update_target(
            warrior_entity,
            warrior,
            warrior_transform,
            target,
            &mut commands,
            players,
            &mut writer,
            &mut rng,
        );
    }
}

fn update_target(
    warrior_entity: Entity,
    warrior: &Warrior,
    warrior_transform: &Transform,
    current_target: Option<&NavigationTarget>,
    commands: &mut Commands,
    players: Query<(Entity, &Transform), With<Player>>,
    writer: &mut MessageWriter<ShootMessage>,
    rng: &mut ThreadRng,
) {
    let mut target_transform: Option<&Transform> = None;
    for (_, player_transform) in players {
        let delta = player_transform.translation.xy() - warrior_transform.translation.xy();
        let distance = delta.length();
        if distance > warrior.detection_radius {
            continue;
        }

        target_transform = Some(player_transform);
        break;
    }

    if target_transform.is_some() {
        commands
            .entity(warrior_entity)
            .insert(NavigationTarget(target_transform.unwrap().translation.xy()));

        writer.write(ShootMessage { entity: warrior_entity });
        
        return;
    }

    if current_target.is_some() {
        return;
    }

    //pick a random point relative to current position and travel there
    let current_pos = warrior_transform.translation.truncate();
    let offset_x = rng.random_range(-800.0..800.0);
    let offset_y = rng.random_range(-800.0..800.0);
    let target = current_pos + Vec2::new(offset_x, offset_y);
    commands
        .entity(warrior_entity)
        .insert(NavigationTarget(target));
}

pub fn warrior_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut warriors: Query<(
        Entity,
        &mut Transform,
        &Warrior,
        &mut Velocity,
        &NavigationTarget,
    )>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut transform, warrior, mut velocity, target) in &mut warriors {
        //arrival check
        let current_pos = transform.translation.xy();
        let delta = target.0 - current_pos;
        let distance = delta.length();
        if distance <= WARRIOR_RADIUS {
            commands.entity(entity).remove::<NavigationTarget>();

            continue;
        }

        // acceleration and damping
        let direction = delta.normalize();
        velocity.0 += warrior.acceleration * delta_time * direction;
        velocity.0 *= 0.98;

        // rotation
        if velocity.length_squared() > 1.0 {
            let angle = velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}
