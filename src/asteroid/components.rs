use bevy::asset::AssetPath;
use bevy::math::Vec2;
use bevy::prelude::Component;
use rand::prelude::*;
use std::ops::Range;

#[derive(Component, Debug)]
pub struct AsteroidSpawnArea {
    pub from: Vec2,
    pub to: Vec2,
    pub spawn_angle_range: Range<f32>,
}

#[derive(Component)]
pub struct Asteroid {
    shape: AsteroidShape,
    pub rotation_speed: f32,
}

impl Asteroid {
    pub fn new(shape: &AsteroidShape) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            shape: *shape,
            rotation_speed: rng.gen_range(shape.get_random_rotation_speed()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum AsteroidShape {
    SmallRound,
    BigRound,
    SmallSquare,
    BigSquare,
}

impl<'a> Into<AssetPath<'a>> for AsteroidShape {
    fn into(self) -> AssetPath<'a> {
        match self {
            AsteroidShape::SmallRound => "asteroids/asteroid_small.png".into(),
            AsteroidShape::BigRound => "asteroids/asteroid_big.png".into(),
            AsteroidShape::SmallSquare => "asteroids/asteroid_square_small.png".into(),
            AsteroidShape::BigSquare => "asteroids/asteroid_square_big.png".into(),
        }
    }
}

impl AsteroidShape {
    fn get_random_rotation_speed(&self) -> Range<f32> {
        match self {
            AsteroidShape::SmallRound => 0.75..1.0,
            AsteroidShape::BigRound => 0.3..0.65,
            AsteroidShape::SmallSquare => 0.75..1.0,
            AsteroidShape::BigSquare => 0.3..0.65,
        }
    }
}
