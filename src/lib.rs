pub mod satellite;
pub mod orbits;
pub mod keyboard;
pub mod controls;

use bevy::prelude::*;

#[derive(Resource)]
pub struct TimewarpLevel(pub usize);

#[derive(Resource)]
pub struct TimewarpFactor(pub f64);
