pub mod ui;

use crate::common::ui::UiPlugin;
use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin);
    }
}
