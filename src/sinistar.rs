use std::collections::HashSet;

use bevy::prelude::*;
use rand::prelude::*;

use crate::physics::*;

const SPAWN_DISTANCE: f32 = 200.0;
const PIECE_COUNT: IVec2 = IVec2::new(4, 5);
const PIECE_SIZE: f32 = 25.;

#[derive(Component)]
pub struct Sinistar;

#[derive(Component)]
pub struct SinistarPiece(IVec2);

pub fn spawn_sinistar(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_pieces: Query<(&SinistarPiece, &ChildOf)>,
) {
    let mut rng = rand::rng();
    let angle: f32 = rng.random_range(0.0..360.0);
    let position = Vec3::new(
        SPAWN_DISTANCE * angle.sin(),
        SPAWN_DISTANCE * angle.cos(),
        0.0,
    );
    let parent = commands
        .spawn((
            Sinistar,
            Transform::from_translation(position),
            GlobalTransform::default(),
            Velocity(Vec2::ZERO),
            Collider {
                radius: f32::min(
                    PIECE_COUNT.x as f32 * PIECE_SIZE,
                    PIECE_COUNT.y as f32 * PIECE_SIZE,
                ) / 2.0,
            },
            Mass(20.0),
        ))
        .id();

    add_pieces(&mut commands, parent, &mut meshes, &mut materials, &existing_pieces, None);
}

pub fn add_pieces(
    commands: &mut Commands,
    parent: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    existing_pieces: &Query<(&SinistarPiece, &ChildOf)>,
    max_add_count: Option<u32>,
) {
    if let Some(max_add) = max_add_count
        && max_add == 0
    {
        return;
    }

    let mut occupied = HashSet::new();
    for (slot, child_of) in existing_pieces.iter() {
        if child_of.parent() == parent {
            occupied.insert(slot.0);
        }
    }

    let mesh = meshes.add(Rectangle::new(PIECE_SIZE, PIECE_SIZE));
    let material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.6, 0.6)));
    let mut added_count: u32 = 0;
    for x in 0..PIECE_COUNT.x {
        for y in 0..PIECE_COUNT.y {
            let cell = IVec2::new(x, y);
            if occupied.contains(&cell){
                continue;
            }

            commands.spawn((
                SinistarPiece(IVec2::new(x, y)),
                Transform::from_translation(Vec3::new(
                    (x as f32 - (PIECE_COUNT.x as f32 - 1.0) / 2.0) * PIECE_SIZE,
                    (y as f32 - (PIECE_COUNT.y as f32 - 1.0) / 2.0) * PIECE_SIZE,
                    0.0,
                )),
                ChildOf(parent),
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
            ));

            added_count += 1;
            if let Some(max_add) = max_add_count
                && added_count >= max_add
            {
                return;
            }
        }
    }
}

pub fn remove_pieces(
    commands: &mut Commands,
    parent: Entity,
    pieces: &Query<(Entity, &ChildOf), With<SinistarPiece>>,
    max_remove_count: Option<u32>,
) {
    if let Some(max) = max_remove_count && max == 0 {
        return;
    }

    let mut removed_count = 0;
    for (entity, child_of) in pieces.iter() {
        if child_of.parent() != parent {
            continue;
        }

        commands.entity(entity).despawn();

        removed_count += 1;
        if let Some(max) = max_remove_count && removed_count >= max {
            return;
        }
    }
}