use bevy::prelude::*;

mod bundles;
mod commands;
mod components;
mod constants;
mod systems;

use crate::bullet::BulletPlugin;
use crate::invincible::InvinciblePlugin;
use crate::movement::MovementPlugin;
use crate::warp::WarpPlugin;
use systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (player_input, on_collision_system),
            )
            .add_systems(FixedUpdate, on_invincibility_end_system)
            .add_plugins(WarpPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(InvinciblePlugin)
            .add_plugins(BulletPlugin);
    }
}
