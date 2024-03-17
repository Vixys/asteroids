use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Debug)]
pub struct AsteroidLineSpawner {
    pub points: Vec<Vec2>,
}

impl AsteroidLineSpawner {
    pub fn random_point(&self) -> Vec2 {
        let mut rng = rand::thread_rng();

        let segment_index = rng.gen_range(0..self.points.len());
        let segment_ratio = rng.gen_range(0.0..=1.0);

        self.points[segment_index].lerp(
            self.points[(segment_index + 1) % self.points.len()],
            segment_ratio,
        )
    }
}
