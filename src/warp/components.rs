use bevy::prelude::*;

#[derive(Component)]
pub struct Warp;

#[derive(Component)]
pub struct WarpArea {
    pub area: Rect,
}
