use bevy::prelude::*;

mod components;
mod systems;
mod bundles;

use systems::*;
use crate::warp::WarpPlugin;
use crate::movement::MovementPlugin;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_input)
            .add_plugins(WarpPlugin)
            .add_plugins(MovementPlugin);
    }
}