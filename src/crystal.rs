use bevy::prelude::*;
use rand::prelude::*;
use crate::physics::*;

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