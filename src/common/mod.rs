pub mod movement;
pub mod ui;
pub mod warp;

use crate::common::movement::MovementPlugin;
use crate::common::ui::UiPlugin;
use crate::common::warp::WarpPlugin;
use bevy::prelude::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(WarpPlugin);
    }
}
