use bevy::prelude::*;

mod bundles;
mod components;
mod events;
mod systems;

use crate::bullet::BulletPlugin;
use crate::movement::MovementPlugin;
use crate::warp::WarpPlugin;
use systems::*;

use self::events::SpawnPlayerEvent;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (player_input, on_collistion_system, on_spawn_player_system),
            )
            .add_plugins(WarpPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(BulletPlugin)
            .add_event::<SpawnPlayerEvent>();
    }
}
