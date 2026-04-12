use core::f32;

use bevy::prelude::*;

use crate::{audio::*, health::*, physics::*};

const SPEED: f32 = 10.;

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
pub struct Sinibomb {
    pub radius: f32,
    pub damage: i32,
}

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
                Transform::from_translation(transform.translation),
                Sinibomb {
                    radius: 5.0,
                    damage: 1,
                },
                Velocity(Vec2::ZERO),
                Mesh2d(meshes.add(Circle::new(5.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 1.0)))),
            ));

            commands.spawn(AudioPlayer::new(sounds.shoot.clone()));
            sinibombs.count -= 1;

            launcher.timer = launcher.cooldown;
        }
    }
}
