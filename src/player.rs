use bevy::prelude::*;

use crate::{health::*, physics::*, shooting::*, team::*};

#[derive(Component)]
pub struct Player
{
    pub speed: f32,
}

pub fn spawn_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
        commands.spawn((
        Player
        {
            speed: 500.0
        },
        Velocity(Vec2::ZERO),
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.3),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        Collider { radius: 15.0 },
        Mass(10.0),
        Mesh2d(meshes.add(Circle::new(15.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.0, 0.0, 1.0)))),
        Gun {
            cooldown: 0.5,
            timer: 0.0,
            projectile_speed: 1000.0,
        },
        Health(1),
        Team::Player,
    ));
}