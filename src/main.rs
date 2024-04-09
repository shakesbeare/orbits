use bevy::math::f64::DVec3;
use bevy::prelude::*;
use big_space::{FloatingOrigin, GridCell};

use orbits::satellite::{
    interact_satellites, update_satellite, update_transforms, Satellite, Star,
};

use orbits::TimewarpFactor;
use orbits::TimewarpLevel;

const MASS_EARTH: f32 = 5.97e24; // kilograms
const RADIUS_EARTH: f32 = 6378.0 * 1000.0; // meters

const MASS_MOON: f32 = 7.35e22; // kilograms
const RADIUS_MOON: f32 = 1737.0 * 1000.0; // meters

const MASS_SUN: f32 = 1.989e30; // kilograms
const RADIUS_SUN: f32 = 696_340.0 * 1000.0; // meters

const DISTANCE_EARTH_MOON: f32 = 384_400.0 * 1000.0; // meters
const DISTANCE_EARTH_SUN: f32 = 149_600_000.0 * 1000.0; // meters

fn main() {
    App::new()
        .add_plugins(( 
            DefaultPlugins.build().disable::<TransformPlugin>(),
            big_space::FloatingOriginPlugin::<i128>::default(),
        ))
        .insert_resource(orbits::satellite::InfoTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(TimewarpLevel(0))
        .insert_resource(TimewarpFactor(1.0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_satellite,
                update_transforms,
                interact_satellites,
                position_camera,
                orbits::satellite::get_satellite_info,
                orbits::controls::handle_camera_move_input,
                orbits::controls::handle_timewarp_input,
                // orbits::controls::print_scan_codes,
            ),
        )
        // .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
        // .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let distance_earth_moon = (radius_earth) + 370.0 * 1000.0; // meters

    let earth_velocity = (MASS_SUN as f64
        * orbits::satellite::GRAVITY_CONSTANT
        / DISTANCE_EARTH_SUN as f64)
        .sqrt();

    let moon_velocity = (MASS_EARTH as f64
        * orbits::satellite::GRAVITY_CONSTANT
        / DISTANCE_EARTH_MOON as f64)
        .sqrt()
        + earth_velocity;

    let mesh = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 1.0,
            subdivisions: 4,
        })
        .unwrap(),
    );

    // Create the camera
    commands.spawn((
        orbits::controls::ControlScheme::wasd(),
        orbits::controls::MoveVector(Vec3::ZERO),
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(
                0.0, 0.0, RADIUS_SUN,
            ))
            .looking_at(Vec3::ZERO, Vec3::Y),
            projection: PerspectiveProjection {
                far: f32::MAX,
                ..Default::default()
            }
            .into(),
            ..default()
        },
        FloatingOrigin,
        GridCell::<i128>::default(),
    ));

    // Create the Sun
    commands.spawn((
        Satellite {
            mass: MASS_SUN as f64,
            velocity: DVec3::new(0.0, 0.0, 0.0),
            position: DVec3::new(0.0, 0.0, 0.0),
            acceleration: DVec3::new(0.0, 0.0, 0.0),
            name: "Sun".to_string(),
        },
        PbrBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(RADIUS_SUN),
                ..default()
            },
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::ORANGE_RED,
                emissive: (Color::ORANGE_RED * 2.),
                ..default()
            }),
            ..default()
        },
        GridCell::<i128>::default(),
    ));

    // Create the Earth
    commands.spawn((
        Satellite {
            mass: MASS_EARTH as f64,
            velocity: DVec3::new(0.0, earth_velocity, 0.0),
            position: DVec3::new(DISTANCE_EARTH_SUN as f64, 0.0, 0.0),
            acceleration: DVec3::new(0.0, 0.0, 0.0),
            name: "Earth".to_string(),
        },
        PbrBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(RADIUS_EARTH),
                ..default()
            },
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            ..default()
        },
        GridCell::<i128>::default(),
    ));

    // Create the moon
    commands.spawn((
        Satellite {
            mass: MASS_MOON as f64,
            velocity: DVec3::new(0.0, moon_velocity, 0.0),
            position: DVec3::new(
                DISTANCE_EARTH_SUN as f64 + DISTANCE_EARTH_MOON as f64,
                0.0,
                0.0,
            ),
            acceleration: DVec3::new(0.0, 0.0, 0.0),
            name: "Moon".to_string(),
        },
        PbrBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::splat(RADIUS_MOON),
                ..default()
            },
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..default()
            }),
            ..default()
        },
    ));

}

pub fn position_camera(
    planets: Query<
        (&Satellite, &Transform),
        Without<orbits::controls::MoveVector>,
    >,
    mut query: Query<(&mut Transform, &orbits::controls::MoveVector)>,
) {
    let mut planet_transform = Vec3::new(0.0, 0.0, 0.0);

    for planet in planets.iter() {
        if planet.0.name == "Sun" {
            planet_transform = planet.1.translation;
        }
    }

    let (mut transform, mv) = query.single_mut();
    let move_vec_scaled = Vec3::new(
        mv.0.x * 1_000.0 * 1000.0 * 0.0,
        mv.0.y * 1_000.0 * 1000.0 * 0.0,
        mv.0.z * 10_000.0 * 1000.0,
    );
    let translation = transform.translation + move_vec_scaled;
    // transform.translation = translation;
    transform.translation = Vec3::new(
        planet_transform.x,
        planet_transform.y,
        planet_transform.z + translation.z,
    );

    if mv.0.z != 0.0 {
        dbg!(transform.translation);
        match mv.0.z as i32 {
            1 => {
                dbg!("zoom in");
            }
            -1 => {
                dbg!("zoom out");
            }
            _ => {}
        }
    }
}
