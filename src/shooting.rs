use bevy::prelude::*;

use crate::{health::*, physics::*, teams::*, asteroid::*, crystal::*};

#[derive(Component)]
pub struct Gun {
    pub cooldown: f32,
    pub timer: f32,
    pub projectile_speed: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub lifetime: f32,
    pub radius: f32,
    pub damage: i32,
    pub team: Team,
}

#[derive(Message)]
pub struct ShootMessage {
    pub entity: Entity,
}

pub fn gun_system(
    mut commands: Commands,
    time: Res<Time>,
    mut messages: MessageReader<ShootMessage>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &Transform, &mut Gun, &Team)>,
) {
    for (_, _, mut gun, _) in &mut query {
        gun.timer -= time.delta_secs();
    }

    for message in messages.read() {
        if let Ok((_, transform, mut gun, team)) = query.get_mut(message.entity) {
            if gun.timer > 0.0 {
                continue;
            }

            let forward = (transform.rotation * Vec3::Y).truncate();
            let velocity = forward * gun.projectile_speed;
            commands.spawn((
                Transform::from_translation(transform.translation),
                Projectile {
                    lifetime: 1.0,
                    radius: 5.0,
                    damage: 1,
                    team: *team,
                },
                Velocity(velocity),
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0)))),
            ));

            gun.timer = gun.cooldown;
        }
    }
}

pub fn projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut projectiles: Query<(Entity, &Transform, &mut Projectile, &Velocity)>,
    mut targets: Query<(Entity, &Transform, &Collider, &mut Health, &Team, Option<&Asteroid>)>, 
) {
    for (projectile_entity, proj_transform, mut projectile, proj_vel) in &mut projectiles {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(projectile_entity).despawn();
            continue;
        }

        let proj_pos = proj_transform.translation.truncate();
        
        let mut hit_something = false;

        for (target_entity, target_transform, target_collider, mut target_health, target_team, asteroid_opt) in &mut targets {
            if projectile.team == Team::None || projectile.team == *target_team {
                continue;
            }

            let target_pos = target_transform.translation.truncate();
            let dist = proj_pos.distance(target_pos);
            let min_dist = projectile.radius + target_collider.radius;

            if dist < min_dist {
                //despawn bullet
                commands.entity(projectile_entity).despawn();
                hit_something = true;

                //damage target
                target_health.0 -= projectile.damage;

                if asteroid_opt.is_some() {
                    let impact_dir = proj_vel.0.normalize_or_zero() * 50.0;
                    spawn_crystal(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        target_transform.translation,
                        impact_dir
                    );
                }
                
                if target_health.0 <= 0 { 
                    commands.entity(target_entity).despawn();
                }

                break; 
            }
        }
        
        if hit_something {
            continue;
        }
    }
}