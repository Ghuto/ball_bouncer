use bevy::prelude::*;

use crate::states::GameState;

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[source(GameState = GameState::InMenu)]
pub enum MenuPage {
    #[default]
    Main,
}
