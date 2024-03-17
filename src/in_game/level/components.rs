use bevy::prelude::*;

#[derive(Component)]
pub struct LevelUiMarker;

#[derive(Component)]
pub struct AsteroidLevel {
    pub id: u32,
    pub spawning_timer: Timer,
    pub max_asteroids: u32,
    pub total_asteroids: u32,
    pub initial_asteroids: u32,
    pub asteroid_max_speed: f32,
}

impl Default for AsteroidLevel {
    fn default() -> Self {
        AsteroidLevel {
            id: 1,
            spawning_timer: Timer::from_seconds(5.0, TimerMode::Once),
            max_asteroids: 5,
            total_asteroids: 10,
            initial_asteroids: 5,
            asteroid_max_speed: 50.0,
        }
    }
}

impl AsteroidLevel {
    pub fn next_level(&mut self) {
        self.id += 1;
        self.spawning_timer = Timer::from_seconds(self.spawning_timer.duration().as_secs_f32() * 0.9, TimerMode::Once);
        self.max_asteroids += 2;
        self.total_asteroids += 5;
        self.initial_asteroids += 2;
        self.asteroid_max_speed *= 1.1;
    }
}
