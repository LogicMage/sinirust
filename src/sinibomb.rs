use bevy::prelude::*;

use crate::{audio::*, physics::*, sinistar::*, team::Team};

const RADIUS: f32 = 5.;
const SPEED: f32 = 250.;

#[derive(Resource, Default)]
pub struct Sinibombs {
    pub count: i32,
}

#[derive(Component)]
pub struct Launcher {
    pub cooldown: f32,
    pub timer: f32,
}

#[derive(Component)]
pub struct Sinibomb;

#[derive(Message)]
pub struct LaunchMessage {
    pub entity: Entity,
}

pub fn launcher_system(
    mut commands: Commands,
    time: Res<Time>,
    mut sinibombs: ResMut<Sinibombs>,
    mut messages: MessageReader<LaunchMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(&Transform, &mut Launcher)>,
    sounds: Res<AudioAssets>,
) {
    for (_, mut launcher) in &mut query {
        launcher.timer -= time.delta_secs();
    }

    if sinibombs.count == 0 {
        return;
    }

    for message in messages.read() {
        if let Ok((transform, mut launcher)) = query.get_mut(message.entity) {
            if launcher.timer > 0.0 {
                continue;
            }

            commands.spawn((
                Sinibomb,
                Transform::from_translation(transform.translation),
                Velocity(Vec2::ZERO),
                Mesh2d(meshes.add(Circle::new(RADIUS))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 1.0)))),
            ));

            commands.spawn(AudioPlayer::new(sounds.launch.clone()));
            sinibombs.count -= 1;

            launcher.timer = launcher.cooldown;
        }
    }
}

pub fn bomb_system(
    mut commands: Commands,
    mut sinibombs: Query<(Entity, &Transform, &mut Velocity), With<Sinibomb>>,
    sinistars: Query<(Entity, &Sinistar, &Transform, &Collider)>,
    colliders: Query<(Entity, &Transform, &Collider)>,
    pieces: Query<(Entity, &ChildOf), With<SinistarPiece>>,
    teams: Query<&Team>,
) {
    for (sinibomb_entity, bomb_transform, mut bomb_velocity) in &mut sinibombs {
        let bomb_position = bomb_transform.translation.xy();
        let mut closest: Option<(Entity, Vec2)> = None;
        let mut closest_dist_sq = f32::MAX;
        for (sin_entity, _, sin_transform, _) in &sinistars {
            let sin_pos = sin_transform.translation.xy();
            let dist_sq = bomb_position.distance_squared(sin_pos);

            if dist_sq < closest_dist_sq {
                closest_dist_sq = dist_sq;
                closest = Some((sin_entity, sin_pos));
            }
        }

        if let Some((_, target_pos)) = closest {
            let direction = (target_pos - bomb_position).normalize_or_zero();
            bomb_velocity.0 = SPEED * direction;
        }

        for (collider_entity, col_transform, collider) in &colliders {
            let col_pos = col_transform.translation.xy();
            let dist_sq = bomb_position.distance_squared(col_pos);
            if dist_sq <= collider.radius * collider.radius {
                let should_despawn = match teams.get(collider_entity) {
                    Ok(team) => *team != Team::Player,
                    Err(_) => true,
                };
                if !should_despawn {
                    continue;
                }

                if let Ok((sin_entity, _, _, _)) = sinistars.get(collider_entity) {
                    remove_pieces(&mut commands, sin_entity, &pieces, Some(1));
                }
                commands.entity(sinibomb_entity).despawn();

                break;
            }
        }
    }
}
