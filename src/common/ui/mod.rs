pub mod button;

use crate::common::ui::button::button_system;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}
