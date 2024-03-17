use bevy::prelude::*;

use crate::game_state::GameState;
use systems::*;

use self::events::InvincibleEndEvent;

pub mod components;
pub mod events;
mod systems;

pub struct InvinciblePlugin;

impl Plugin for InvinciblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, fade_invincible.run_if(in_state(GameState::InGame)))
            .add_event::<InvincibleEndEvent>();
    }
}
