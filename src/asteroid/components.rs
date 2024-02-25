use std::ops::{Range, RangeInclusive};

use bevy::asset::AssetPath;
use bevy::math::Vec2;
use bevy::prelude::Component;
use rand::distributions::Standard;
use rand::prelude::*;

#[derive(Component, Debug)]
pub struct AsteroidSpawnArea {
    pub from: Vec2,
    pub to: Vec2,
    pub spawn_angle_range: Range<f32>,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Asteroid {
    pub shape: AsteroidShape,
    pub size: AsteroidSize,
}

impl Default for Asteroid {
    fn default() -> Self {
        Self {
            shape: AsteroidShape::Round,
            size: AsteroidSize::Big,
        }
    }
}

impl Asteroid {
    pub fn get_asset_path(&self) -> AssetPath {
        self.into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AsteroidSize {
    Small,
    Medium,
    Big,
}

impl AsteroidSize {
    pub fn get_speed_range(&self) -> RangeInclusive<f32> {
        match self {
            AsteroidSize::Small => 0.75..=1.15,
            AsteroidSize::Medium => 0.55..=0.85,
            AsteroidSize::Big => 0.45..=0.65,
        }
    }

    pub fn get_angular_speed_range(&self) -> RangeInclusive<f32> {
        match self {
            AsteroidSize::Small => 0.75..=1.15,
            AsteroidSize::Medium => 0.55..=0.85,
            AsteroidSize::Big => 0.45..=0.65,
        }
    }

    pub fn get_radius_range(&self) -> RangeInclusive<f32> {
        match self {
            AsteroidSize::Small => 16.0..=24.0,
            AsteroidSize::Medium => 32.0..=48.0,
            AsteroidSize::Big => 64.0..=98.0,
        }
    }

    pub fn shrink(&self) -> Option<AsteroidSize> {
        match self {
            AsteroidSize::Big => Some(AsteroidSize::Medium),
            AsteroidSize::Medium => Some(AsteroidSize::Small),
            _ => None,
        }
    }
}

impl Distribution<AsteroidSize> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> AsteroidSize {
        match rng.gen_range(0..3) {
            0 => AsteroidSize::Small,
            1 => AsteroidSize::Medium,
            _ => AsteroidSize::Big,
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
            AsteroidShape::Round
                if val.size == AsteroidSize::Big || val.size == AsteroidSize::Medium =>
            {
                "asteroids/asteroid_big.png".into()
            }
            AsteroidShape::Round if val.size == AsteroidSize::Small => {
                "asteroids/asteroid_small.png".into()
            }
            AsteroidShape::Square
                if val.size == AsteroidSize::Big || val.size == AsteroidSize::Medium =>
            {
                "asteroids/asteroid_square_big.png".into()
            }
            AsteroidShape::Square if val.size == AsteroidSize::Small => {
                "asteroids/asteroid_square_small.png".into()
            }
            _ => unreachable!(),
        }
    }
}
