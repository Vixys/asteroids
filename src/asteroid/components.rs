use std::ops::Range;

use bevy::asset::AssetPath;
use bevy::math::Vec2;
use bevy::prelude::Component;
use rand::distributions::Standard;
use rand::prelude::*;

use super::constants::*;

#[derive(Component, Debug)]
pub struct AsteroidSpawnArea {
    pub from: Vec2,
    pub to: Vec2,
    pub spawn_angle_range: Range<f32>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Asteroid {
    pub shape: AsteroidShape,
    pub size: f32,
}

impl Default for Asteroid {
    fn default() -> Self {
        Self {
            shape: AsteroidShape::Round,
            size: 2.0_f32.powf((*ASTEROID_SIZE_RANGE.start()) as f32),
        }
    }
}

impl Asteroid {
    pub fn get_angular_velocity(&self) -> f32 {
        let mut rng = rand::thread_rng();
        let indice = self.size.log2() / (*ASTEROID_SIZE_RANGE.start()) as f32;

        ASTEROID_MAX_ANGULAR_VELOCITY / indice
            + rng.gen_range(ASTEROID_ANGULAR_VELOCITY_RANGE_VARIATION)
    }

    pub fn get_asset_path(&self) -> AssetPath {
        self.into()
    }

    pub fn get_scale(&self) -> f32 {
        if self.size > ASTEROID_SWITCH_SIZE {
            self.size / ASTEROID_SWITCH_SIZE
        } else {
            self.size / 32.0
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsteroidShape {
    Round,
    Square,
}

impl Distribution<AsteroidShape> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AsteroidShape {
        match rng.gen_range(0..2) {
            0 => AsteroidShape::Round,
            _ => AsteroidShape::Square,
        }
    }
}

impl<'a> From<&'a Asteroid> for AssetPath<'a> {
    fn from(val: &'a Asteroid) -> Self {
        match val.shape {
            AsteroidShape::Round if val.size >= 48.0 => "asteroids/asteroid_big.png".into(),
            AsteroidShape::Round if val.size < 48.0 => "asteroids/asteroid_small.png".into(),
            AsteroidShape::Square if val.size >= 48.0 => "asteroids/asteroid_square_big.png".into(),
            AsteroidShape::Square if val.size < 48.0 => {
                "asteroids/asteroid_square_small.png".into()
            }
            _ => unreachable!(),
        }
    }
}
