use bevy::prelude::*;
use rand::prelude::*;
use crate::physics::*;
use crate::player::*;
use crate::includes::*;

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
    crystal_query: Query<(Entity, &Transform), With<Crystal>>,
) {
    for player_transform in player_query.iter() {
        
        for (crystal_entity, crystal_transform) in crystal_query.iter() {
            let distance = player_transform.translation.distance(crystal_transform.translation);

            if distance < 30.0 {
                score.0 += 200;
                sinibombs.0 += 1;
                commands.entity(crystal_entity).despawn();
            }
        }
    }
}