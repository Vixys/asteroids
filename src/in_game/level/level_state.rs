use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum LevelState {
    #[default]
    Init,
    Start,
    InProgress,
    End,
}
