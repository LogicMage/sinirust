use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Component)]
pub struct Mass(pub f32);

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += Vec3::new(velocity.x, velocity.y, 0.0) * time.delta_secs();
    }
}

pub fn handle_collisions(mut query: Query<(&mut Transform, &mut Velocity, &Collider, &Mass)>) {
    //iter_combinations_mut ensures we check A vs B, but not A vs A or B vs A again
    let mut combinations = query.iter_combinations_mut();

    while let Some([(mut t1, mut v1, c1, m1), (mut t2, mut v2, c2, m2)]) = combinations.fetch_next()
    {
        let p1 = t1.translation.truncate();
        let p2 = t2.translation.truncate();

        let distance = p1.distance(p2);
        let min_dist = c1.radius + c2.radius;

        if distance < min_dist {
            let normal = (p2 - p1).normalize_or_zero();

            let depth = min_dist - distance;
            let separation = normal * (depth / 2.0);

            t1.translation -= separation.extend(0.0);
            t2.translation += separation.extend(0.0);

            let v_rel = v1.0 - v2.0;
            let vel_along_normal = v_rel.dot(normal);

            if vel_along_normal > 0.0 {
                continue;
            }

            let j = -(2.0 * vel_along_normal) / (1.0 / m1.0 + 1.0 / m2.0);
            let impulse = j * normal;

            v1.0 += impulse / m1.0;
            v2.0 -= impulse / m2.0;
        }
    }
}