use crate::physics::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Gun {
    pub cooldown: f32,
    pub timer: f32,
    pub projectile_speed: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub lifetime: f32,
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
    mut query: Query<(Entity, &Transform, &mut Gun)>,
) {
    // tick cooldowns
    for (_, _, mut weapon) in &mut query {
        weapon.timer -= time.delta_secs();
    }

    for message in messages.read() {
        if let Ok((_, transform, mut gun)) = query.get_mut(message.entity) {
            if gun.timer > 0.0 {
                continue;
            }

            gun.timer = gun.cooldown;

            let forward = (transform.rotation * Vec3::Y).truncate();
            let velocity = forward * gun.projectile_speed;

            commands.spawn((
                Transform::from_translation(transform.translation),
                Projectile { lifetime: 1.0 },
                Velocity(velocity),
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0)))),
            ));
        }
    }
}

pub fn projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut proj) in &mut query {
        proj.lifetime -= time.delta_secs();
        if proj.lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
