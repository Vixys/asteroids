use bevy::prelude::*;

mod systems;
pub mod components;

use systems::*;

pub struct WarpPlugin;

impl Plugin for WarpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, warp_system);
    }
}
