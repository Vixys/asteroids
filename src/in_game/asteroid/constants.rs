use std::f32::consts::{FRAC_PI_4, PI};
use std::ops::RangeInclusive;

pub const ASTEROID_MAX_SPEED: f32 = 100.0;
pub const ASTEROID_PADDING_SPAWN_LOCATION: f32 = 150.0;
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = PI;
pub const INITIAL_ASTEROID_COUNT: usize = 1;
pub const ASTEROID_DIRECTION_NOISE_RANGE: RangeInclusive<f32> = -FRAC_PI_4..=FRAC_PI_4;
