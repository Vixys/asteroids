use bevy::prelude::*;

use super::level_state::LevelState;
// Timer resource that contains between LevelState and the next state
#[derive(Resource)]
pub struct LevelStateTimer {
    pub timer: Option<Timer>,
    pub next_state: Option<LevelState>,
}

impl Default for LevelStateTimer {
    fn default() -> Self {
        LevelStateTimer { timer: None, next_state: None }
    }
}

impl LevelStateTimer {
    pub fn next_state(&mut self, next_state: LevelState) {
        self.timer = Some(Timer::from_seconds(2.5, TimerMode::Once));
        self.next_state = Some(next_state);
    }

    pub fn reset(&mut self) {
        self.timer = None;
        self.next_state = None;
    }
}
