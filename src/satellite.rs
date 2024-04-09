use bevy::prelude::*;
use bevy::math::f64::DVec3;

pub const GRAVITY_CONSTANT: f64 = 6.67e-11;

#[derive(Resource)]
pub struct InfoTimer(pub Timer);

#[derive(Component)]
pub struct Star;

#[derive(Component, Debug)]
pub struct Satellite {
    pub mass: f64,
    pub velocity: DVec3,
    pub position: DVec3,
    pub acceleration: DVec3,
    pub name: String,
}

pub fn interact_satellites(mut query: Query<&mut Satellite>) {
    let mut iter = query.iter_combinations_mut();

    while let Some([mut sat1, mut sat2]) = iter.fetch_next() {
        let r = sat1.position - sat2.position;
    let f_magnitude = GRAVITY_CONSTANT * sat1.mass * sat2.mass / r.length_squared();
        let f = f_magnitude * r.normalize();
        let m1 = sat1.mass;
        let m2 = sat2.mass;

        sat1.acceleration = -1.0 * f / m1;
        sat2.acceleration = f / m2;
    }
}

pub fn update_satellite(time: Res<Time>, timewarp_factor: Res<crate::TimewarpFactor>, mut query: Query<&mut Satellite>) {

    let dt = time.delta_seconds() as f64 * timewarp_factor.0;

    for mut satellite in query.iter_mut() {
        let a = satellite.acceleration;
        satellite.velocity += a * dt;
        let v = satellite.velocity;

        let dx = v * dt + 0.5 * a * dt * dt;
        satellite.position += dx;
    }
}

pub fn update_transforms(mut query: Query<(&mut Transform, &Satellite)>) {
    for (mut transform, satellite) in query.iter_mut() {
        let pos = Vec3::new(
            satellite.position.x as f32,
            satellite.position.y as f32,
            satellite.position.z as f32,
        );
        transform.translation = pos;
    }
}

pub fn get_satellite_info(time: Res<Time>, mut timer: ResMut<InfoTimer>, query: Query<&Satellite>) {
    if timer.0.tick(std::time::Duration::from_secs_f32(time.delta_seconds())).just_finished() {
        for satellite in query.iter() {
            let name = satellite.name.clone();
            let a = satellite.acceleration;
            let v = satellite.velocity;
            let p = satellite.position;
            println!("{name}: a: {a}, v: {v}, p: {p}");
        }
    }
}
