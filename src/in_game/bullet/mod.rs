use bevy::prelude::*;

use crate::game_state::GameState;
use systems::*;

pub mod bundles;
pub mod commands;
mod components;
mod constants;
mod systems;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_collision.run_if(in_state(GameState::InGame)),
                despawn_out_of_bounds.run_if(in_state(GameState::InGame)),
            ),
        );
    }
}
