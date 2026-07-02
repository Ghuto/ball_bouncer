use bevy::prelude::*;

use crate::{game_state::GameState, level::Level, ui_pages::MenuPage};

#[derive(Event)]
pub struct ResumeLevel;

impl ResumeLevel {
    pub fn on_trigger(
        _: On<Self>,
        mut game_state: ResMut<NextState<GameState>>,
        mut page_state: ResMut<NextState<MenuPage>>,
    ) {
        game_state.set(GameState::Running);
        page_state.set(MenuPage::Overlay);
    }
}

#[derive(Event)]
pub struct RestartLevel;

impl RestartLevel {
    pub fn on_trigger(
        _: On<Self>,
        mut commands: Commands,
        mut next_game_state: ResMut<NextState<GameState>>,
        level: Single<(Entity, &Level)>,
    ) {
        // remove current level
        commands.entity(level.0).despawn();

        // trigger an event that creates same level
        commands.trigger(level.1.clone());

        next_game_state.set(GameState::WaitingForInput);
    }
}
