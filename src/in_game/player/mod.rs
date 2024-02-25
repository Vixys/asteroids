use bevy::prelude::*;

use crate::game_state::GameState;
use systems::*;

use crate::in_game::blink::BlinkPlugin;
use crate::in_game::bullet::BulletPlugin;
use crate::in_game::invincible::InvinciblePlugin;
use crate::in_game::movement::MovementPlugin;
use crate::in_game::warp::WarpPlugin;

mod bundles;
mod commands;
mod components;
mod constants;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(
                Update,
                (
                    player_input.run_if(in_state(GameState::InGame)),
                    on_collision_system.run_if(in_state(GameState::InGame)),
                ),
            )
            .add_systems(
                FixedUpdate,
                on_invincibility_end_system.run_if(in_state(GameState::InGame)),
            )
            .add_plugins(WarpPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(InvinciblePlugin)
            .add_plugins(BlinkPlugin)
            .add_plugins(BulletPlugin);
    }
}
