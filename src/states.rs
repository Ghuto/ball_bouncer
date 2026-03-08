use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum GameState {
    #[default]
    InMenu,
    Playing,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[source(GameState = GameState::Playing)]
pub enum PausedState {
    #[default]
    Playing,
    Paused,
}
