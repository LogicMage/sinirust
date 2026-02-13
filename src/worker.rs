use bevy::prelude::*;
use crate::{health::*, navigation::*, physics::*, team::*, crystal::*};
use rand::prelude::*;

#[derive(Component)]
pub struct Worker;

#[derive(Component, Default)]
pub struct HasCrystal(pub bool);

//worker state machine
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WorkerState {
    #[default]
    Roaming,
    Collecting,
    Returning,
}

#[derive(Component)]
pub struct WorkerStats {
    pub speed: f32,
    pub detection_radius: f32,
}

pub fn spawn_workers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();

    for _ in 0..20 {
        let p_x = rng.random_range(-1000.0..1000.0);
        let p_y = rng.random_range(-1000.0..1000.0);

        commands.spawn((
            Worker,
            WorkerState::default(),
            WorkerStats {
                speed: 200.0,
                detection_radius: 400.0,
            },
            Velocity(Vec2::ZERO),
            Collider { radius: 12.0 },
            Mass(5.0),
            Mesh2d(meshes.add(RegularPolygon::new(12.0, 6))), 
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.9, 0.1, 0.1)))),
            Transform::from_xyz(p_x, p_y, 0.0),
            Health(1),
            Team::Enemy,
            HasCrystal(false)
        ));
    }
}

pub fn worker_roaming_ai(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &WorkerState), (With<Worker>, Without<NavigationTarget>)>,
) {
    let mut rng = rand::rng();

    for (entity, transform, state) in &mut query {
        if let WorkerState::Roaming = state {
            //pick a random point relative to current position and travel there
            let offset_x = rng.random_range(-800.0..800.0);
            let offset_y = rng.random_range(-800.0..800.0);

            let current_pos = transform.translation.truncate();
            let target = current_pos + Vec2::new(offset_x, offset_y);

            commands.entity(entity).insert(NavigationTarget(target));
        }
    }
}

pub fn worker_sensor_ai(
    mut commands: Commands,
    // We must query for &HasCrystal to check the bool, rather than using Without<HasCrystal>
    mut worker_query: Query<(Entity, &Transform, &WorkerStats, &mut WorkerState, &HasCrystal), With<Worker>>,
    crystal_query: Query<&Transform, With<Crystal>>,
) {
    for (entity, worker_tf, stats, mut state, has_crystal) in &mut worker_query {
        // Only look for crystals if we are roaming or already collecting (to update target)
        // AND we don't currently have a crystal.
        if !has_crystal.0 && matches!(*state, WorkerState::Roaming | WorkerState::Collecting) {
            
            let worker_pos = worker_tf.translation.truncate();
            let mut closest_crystal_pos: Option<Vec2> = None;
            
            // The prompt requested a 200 unit detection trigger
            let mut closest_dist = 200.0_f32.min(stats.detection_radius);

            for crystal_tf in &crystal_query {
                let crystal_pos = crystal_tf.translation.truncate();
                let dist = worker_pos.distance(crystal_pos);

                if dist < closest_dist {
                    closest_dist = dist;
                    closest_crystal_pos = Some(crystal_pos);
                }
            }

            if let Some(target_pos) = closest_crystal_pos {
                // Determine logic: State switch
                // If we found a crystal, we are now Collecting
                if *state != WorkerState::Collecting {
                   *state = WorkerState::Collecting;
                }
                
                // Update the moving target to the crystal's position
                commands.entity(entity).insert(NavigationTarget(target_pos));
            } else if *state == WorkerState::Collecting {
                // If we were collecting but can no longer see a crystal (someone else took it),
                // go back to roaming
                *state = WorkerState::Roaming;
                commands.entity(entity).remove::<NavigationTarget>();
            }
        }
    }
}

pub fn worker_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Velocity, &WorkerStats, &NavigationTarget), With<Worker>>,
) {
    let dt = time.delta_secs();

    for (entity, mut transform, mut velocity, stats, target) in &mut query {
        let current_pos = transform.translation.truncate();
        let delta = target.0 - current_pos;
        let distance_sq = delta.length_squared();

        //arrival check
        if distance_sq < 400.0 {
            commands.entity(entity).remove::<NavigationTarget>();
            velocity.0 *= 0.9; //slow until we reach the target
            continue;
        }

        //movement logic
        let direction = delta.normalize();
        velocity.0 += direction * stats.speed * dt;
        velocity.0 *= 0.98;

        //visual rotation
        if velocity.length_squared() > 1.0 {
            let angle = velocity.y.atan2(velocity.x) - std::f32::consts::FRAC_PI_2;
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}