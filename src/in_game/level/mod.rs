use bevy::prelude::*;

use level_state::LevelState;

use crate::game_state::GameState;

use self::resources::LevelStateTimer;
use self::systems::*;

mod components;
mod level_state;
mod resources;
mod systems;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>()
            .insert_resource(LevelStateTimer::default())
            .add_systems(OnEnter(GameState::InGame), setup_level)
            .add_systems(OnEnter(LevelState::Start), start_level_ui)
            .add_systems(Update, (update_level_state, spawn_asteroid.run_if(in_state(LevelState::InProgress))))
            .add_systems(OnEnter(LevelState::InProgress), spawn_initial_asteroids)
            .add_systems(FixedPostUpdate, level_completed.run_if(in_state(LevelState::InProgress)));
    }
}
