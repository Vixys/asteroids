use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct WarpPlugin;

impl Plugin for WarpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, warp_system);
    }
}
