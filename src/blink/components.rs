use bevy::{prelude::*, time::Timer};

#[derive(Component, Debug)]
pub struct Blink {
    pub timer: Timer,
}

impl Default for Blink {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.35, TimerMode::Repeating),
        }
    }
}
