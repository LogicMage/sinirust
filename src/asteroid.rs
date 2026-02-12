use bevy::prelude::*;
use rand::prelude::*;
use bevy::math::*;

use crate::physics::*;
use crate::includes::*;

use crate::health::*;
use crate::teams::*;

#[derive(Component)]
pub struct Asteroid;

pub fn spawn_asteroids(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>)
{
    let mut rng = rand::rng();

    for _i in 0..100 {
        let half_w = WORLD_WIDTH / 2.0;
        let half_h = WORLD_HEIGHT / 2.0;
        
        let p_x: f32 = rng.random_range(-half_w..half_w);
        let p_y: f32 = rng.random_range(-half_h..half_h);
        
        let v_x: f32 = rng.random_range(-20.0..20.0);
        let v_y: f32 = rng.random_range(-20.0..20.0);
        
        commands.spawn((
            Asteroid,
            Velocity(Vec2::new(v_x, v_y)),
            WrapsAroundCamera,
            Transform {
                translation: Vec3::new(p_x, p_y, 9.0),
                ..default()
            },
            Collider { radius: 30.0 },
            Mass(50.0),
            Mesh2d(meshes.add(Circle::new(30.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(Color::srgb(0.5, 0.5, 0.5)))),
            Health(3),
            Team::None
        ));
    }
}