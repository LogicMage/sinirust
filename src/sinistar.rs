use std::{collections::HashSet, u32};

use bevy::prelude::*;
use rand::prelude::*;

use crate::{audio::AudioAssets, physics::*, player::*};

const SPAWN_DISTANCE: f32 = 500.0;
const PIECE_COUNT: IVec2 = IVec2::new(2, 2);
const PIECE_SIZE: f32 = 25.0;
const RADIUS: f32 = f32::min(
    PIECE_COUNT.x as f32 * PIECE_SIZE,
    PIECE_COUNT.y as f32 * PIECE_SIZE,
) / 2.0;
const ATTACK_RADIUS: f32 = RADIUS + 20.0;
const SPEED: f32 = 300.0;

#[derive(Component)]
pub struct Sinistar {
    pub active: bool,
}

#[derive(Component)]
pub struct SinistarPiece(IVec2);

pub fn spawn_sinistar(mut commands: Commands) {
    let mut rng = rand::rng();
    let angle: f32 = rng.random_range(0.0_f32..360.0).to_radians();
    let position = Vec3::new(
        SPAWN_DISTANCE * angle.sin(),
        SPAWN_DISTANCE * angle.cos(),
        0.0,
    );
    commands.spawn((
        Sinistar { active: false },
        Transform::from_translation(position),
        GlobalTransform::default(),
        Velocity(Vec2::ZERO),
        Collider { radius: RADIUS },
        Mass(20.0),
    ));
}

pub fn add_pieces(
    commands: &mut Commands,
    sounds: &AudioAssets,
    parent: Entity,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    sinistars: &mut Query<&mut Sinistar>,
    existing_pieces: &Query<(&SinistarPiece, &ChildOf)>,
    max_add_count: Option<u32>,
) {
    let max_add = max_add_count.unwrap_or(u32::MAX);
    if max_add == 0 {
        return;
    }

    let mut occupied = HashSet::new();
    for (slot, child_of) in existing_pieces.iter() {
        if child_of.parent() == parent {
            occupied.insert(slot.0);
        }
    }
    let max_piece_count = (PIECE_COUNT.x * PIECE_COUNT.y) as u32;
    if occupied.len() as u32 == max_piece_count {
        return;
    }

    let mesh = meshes.add(Rectangle::new(PIECE_SIZE, PIECE_SIZE));
    let material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.6, 0.6)));
    let mut added_count: u32 = 0;
    for y in 0..PIECE_COUNT.y {
        for x in 0..PIECE_COUNT.x {
            let cell = IVec2::new(x, y);
            if occupied.contains(&cell) {
                continue;
            }

            let child = commands
                .spawn((
                    SinistarPiece(IVec2::new(x, y)),
                    Transform::from_translation(Vec3::new(
                        (x as f32 - (PIECE_COUNT.x as f32 - 1.0) / 2.0) * PIECE_SIZE,
                        (y as f32 - (PIECE_COUNT.y as f32 - 1.0) / 2.0) * PIECE_SIZE,
                        0.0,
                    )),
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(material.clone()),
                ))
                .id();
            commands.entity(parent).add_child(child);

            added_count += 1;
            if added_count >= max_add {
                break;
            }
        }
        if added_count >= max_add {
            break;
        }
    }

    if added_count == 0 || occupied.len() as u32 + added_count < max_piece_count {
        return;
    }

    if let Ok(mut sinistar) = sinistars.get_mut(parent)
        && !sinistar.active
    {
        sinistar.active = true;
        commands.spawn(AudioPlayer::new(sounds.beware.clone()));
    }
}

pub fn remove_pieces(
    commands: &mut Commands,
    parent: Entity,
    sinistars: &mut Query<&mut Sinistar>,
    pieces: &Query<(Entity, &ChildOf), With<SinistarPiece>>,
    max_remove_count: Option<u32>,
) {
    let max_remove = max_remove_count.unwrap_or(u32::MAX);
    if max_remove == 0 {
        return;
    }

    let mut remaining = 0;
    for (_, child_of) in pieces.iter() {
        if child_of.parent() == parent {
            remaining += 1;
        }
    }
    if remaining == 0 {
        return;
    }

    let mut removed_count = 0;
    for (entity, child_of) in pieces.iter() {
        if child_of.parent() != parent {
            continue;
        }

        commands.entity(entity).despawn();

        removed_count += 1;
        if removed_count >= max_remove {
            break;
        }
    }

    if remaining - removed_count == 0 {
        if let Ok(mut sinistar) = sinistars.get_mut(parent) {
            sinistar.active = false;
        }
    }
}

pub fn sinistar_chase(
    mut commands: Commands,
    time: Res<Time>,
    mut sinistars: Query<(&Sinistar, &Transform, &mut Velocity)>,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    let Ok((player_entity, player_transform)) = players.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();
    for (sinistar, transform, mut velocity) in &mut sinistars {
        if !sinistar.active {
            velocity.0 = Vec2::ZERO;
            continue;
        }

        let sin_position = transform.translation.xy();
        let dist_sq = sin_position.distance_squared(player_pos);
        if dist_sq <= ATTACK_RADIUS * ATTACK_RADIUS {
            commands.entity(player_entity).despawn();
            return;
        }

        let direction = (player_pos - sin_position).normalize();
        velocity.0 += SPEED * time.delta_secs() * direction;
        velocity.0 *= 0.98;
    }
}
