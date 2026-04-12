use bevy::prelude::*;
use rand::prelude::*;

use crate::physics::*;

const SPAWN_DISTANCE: f32 = 200.0;
const PIECE_COUNT: IVec2 = IVec2::new(4, 5);
const PIECE_SIZE: f32 = 25.;

#[derive(Component)]
pub struct Sinistar;

#[derive(Component)]
pub struct SinistarPiece;

pub fn spawn_sinistar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let angle: f32 = rng.random_range(0.0..360.0);
    let position = Vec3::new(SPAWN_DISTANCE * angle.sin(), SPAWN_DISTANCE * angle.cos(), 0.0);
    let parent = commands
        .spawn((
            Sinistar,
            Transform::from_translation(position),
            GlobalTransform::default(),
            Velocity(Vec2::ONE),
            Collider {
                radius: f32::min(
                    PIECE_COUNT.x as f32 * PIECE_SIZE,
                    PIECE_COUNT.y as f32 * PIECE_SIZE,
                ) / 2.0,
            },
            Mass(20.0),
        ))
        .id();

    let mesh = meshes.add(Rectangle::new(PIECE_SIZE, PIECE_SIZE));
    let material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.6, 0.6)));
    spawn_pieces(&mut commands, parent, mesh, material);
}

fn spawn_pieces(
    commands: &mut Commands,
    parent: Entity,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    for x in 0..PIECE_COUNT.x {
        for y in 0..PIECE_COUNT.y {
            commands.spawn((
                SinistarPiece,
                Transform::from_translation(Vec3::new(
                    (x as f32 - (PIECE_COUNT.x as f32 - 1.0) / 2.0) * PIECE_SIZE,
                    (y as f32 - (PIECE_COUNT.y as f32 - 1.0) / 2.0) * PIECE_SIZE,
                    0.0,
                )),
                ChildOf(parent),
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
            ));
        }
    }
}
