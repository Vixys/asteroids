use bevy::{prelude::*, time::Timer};

#[derive(Component, Debug)]
pub struct Invincible {
    pub fade_timer: Timer,
    pub blink_timer: Timer,
}

impl Default for Invincible {
    fn default() -> Self {
        Self {
            fade_timer: Timer::from_seconds(3.0, TimerMode::Once),
            blink_timer: Timer::from_seconds(0.35, TimerMode::Repeating),
        }
    }
}
