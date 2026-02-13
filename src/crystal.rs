use bevy::prelude::*;
use rand::prelude::*;
use crate::physics::*;
use crate::player::*;
use crate::includes::*;
use crate::worker::*;
use crate::navigation::*;
use std::collections::HashSet;

#[derive(Component)]
pub struct Crystal;

pub fn spawn_crystal(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    impact_dir: Vec2,
) {
    let mut rng = rand::rng();
    let random_drift_x = rng.random_range(-50.0..50.0);
    let random_drift_y = rng.random_range(-50.0..50.0);

    commands.spawn((
        Crystal,
        WrapsAroundCamera,
        Velocity(impact_dir + Vec2::new(random_drift_x, random_drift_y)),
        Collider { radius: 8.0 },
        Mass(1.0),
        Transform::from_translation(position),
        Mesh2d(meshes.add(RegularPolygon::new(8.0, 3))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 0.9, 0.0)))),
    ));
}

pub fn crystal_impacts(
    mut commands: Commands,
    mut sinibombs: ResMut<Sinibombs>,
    mut score: ResMut<GameScore>,
    player_query: Query<&Transform, With<Player>>,
    mut worker_query: Query<(Entity, &Transform, &mut HasCrystal, &mut WorkerState), With<Worker>>,
    crystal_query: Query<(Entity, &Transform), With<Crystal>>,
) {
    let mut taken_crystals: HashSet<Entity> = HashSet::new();

    for player_transform in player_query.iter() {
        for (crystal_entity, crystal_transform) in crystal_query.iter() {
            let distance = player_transform.translation.distance(crystal_transform.translation);

            if distance < 30.0 {
                score.0 += 200;
                sinibombs.0 += 1;
                commands.entity(crystal_entity).despawn();
                taken_crystals.insert(crystal_entity);
            }
        }
    }

    for (worker_entity, worker_tf, mut has_crystal, mut state) in &mut worker_query {
        if *state == WorkerState::Collecting {
            
            for (crystal_entity, crystal_tf) in &crystal_query {
                if taken_crystals.contains(&crystal_entity) {
                    continue;
                }

                if worker_tf.translation.distance(crystal_tf.translation) < 30.0 {
                    commands.entity(crystal_entity).despawn();
                    taken_crystals.insert(crystal_entity);

                    has_crystal.0 = true;
                    *state = WorkerState::Returning;

                    commands.entity(worker_entity).remove::<NavigationTarget>();
                    break;
                }
            }
        }
    }
}