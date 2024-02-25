use bevy::prelude::*;

use systems::*;

pub mod bundles;
mod components;
mod systems;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (on_collision, despawn_out_of_bounds));
    }
}
