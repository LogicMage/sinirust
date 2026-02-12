use crate::{health::*, navigation::*, physics::*, team::*};
use bevy::prelude::*;
use rand::prelude::*;

const WARRIOR_RADIUS: f32 = 30.0;

#[derive(Component)]
pub struct Warrior {
    pub acceleration: f32,
    //pub detection_radius: f32,
    pub state: WarriorState,
}

//warrior state machine
#[derive(Default)]
pub enum WarriorState {
    #[default]
    Roaming,
    //Fighting,
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
                state: WarriorState::default(),
            },
            Velocity(Vec2::ZERO),
            Collider { radius: WARRIOR_RADIUS },
            Mass(6.0),
            Mesh2d(meshes.add(Circle::new(WARRIOR_RADIUS))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0)))),
            Health(1),
            Team::Enemy,
        ));
    }
}

pub fn warrior_roaming_ai(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Warrior), Without<NavigationTarget>>,
) {
    let mut rng = rand::rng();

    for (entity, transform, warrior) in &mut query {
        if matches!(warrior.state, WarriorState::Roaming) {
            //pick a random point relative to current position and travel there
            let offset_x = rng.random_range(-800.0..800.0);
            let offset_y = rng.random_range(-800.0..800.0);

            let current_pos = transform.translation.truncate();
            let target = current_pos + Vec2::new(offset_x, offset_y);

            commands.entity(entity).insert(NavigationTarget(target));
        }
    }
}

pub fn warrior_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Warrior, &mut Velocity, &NavigationTarget)>,
) {
    let delta_time = time.delta_secs();

    for (entity, mut transform, warrior, mut velocity, target) in &mut query {
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