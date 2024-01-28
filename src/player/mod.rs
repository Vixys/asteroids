use bevy::prelude::*;

mod bundles;
mod components;
mod systems;

use crate::movement::MovementPlugin;
use crate::warp::WarpPlugin;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, player_input)
            .add_plugins(WarpPlugin)
            .add_plugins(MovementPlugin);
    }
}
