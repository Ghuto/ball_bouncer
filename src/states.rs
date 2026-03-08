use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    #[default]
    InMenu,
    Playing,
}
