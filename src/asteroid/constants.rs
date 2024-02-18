use std::f32::consts::{FRAC_PI_4, PI};
use std::ops::{Range, RangeInclusive};

pub const ASTEROID_MAX_SPEED: f32 = 100.0;
pub const ASTEROID_PADDING_SPAWN_LOCATION: f32 = 150.0;
pub const ASTEROID_MAX_ANGULAR_VELOCITY: f32 = PI * 2.0;
pub const ASTEROID_ANGULAR_VELOCITY_RANGE_VARIATION: RangeInclusive<f32> = -FRAC_PI_4..=FRAC_PI_4;
pub const ASTEROID_SIZE_RANGE: RangeInclusive<u32> = 4..=7;
pub const ASTEROID_SPEED_RANGE: Range<f32> = 0.65..1.1;
pub const ASTEROID_SWITCH_SIZE: f32 = 48.0;
