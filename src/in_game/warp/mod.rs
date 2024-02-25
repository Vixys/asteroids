use bevy::prelude::*;

pub mod components;
mod systems;

use crate::game_state::GameState;
use systems::*;

pub struct WarpPlugin;

impl Plugin for WarpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, warp_system.run_if(in_state(GameState::InGame)));
    }
}
