use bevy::prelude::*;

use systems::*;

use self::events::InvincibleEndEvent;

pub mod components;
pub mod events;
mod systems;

pub struct InvinciblePlugin;

impl Plugin for InvinciblePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (fade_invincible, blink_invincible, invincible_end_system),
        )
        .add_event::<InvincibleEndEvent>();
    }
}
