use bevy::prelude::*;

use crate::{
    game_state::{GameState, MainState},
    ui_pages::MenuPage,
};

#[derive(Event)]
pub struct LevelFailed;

pub fn on_level_failed(
    _: On<LevelFailed>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(GameState::Stopped);
    page_state.set(MenuPage::LevelPaused);
}

#[derive(Event)]
pub struct LevelPause;

pub fn on_level_pause(
    _: On<LevelPause>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(GameState::Stopped);
    page_state.set(MenuPage::LevelPaused);
}

#[derive(Event)]
pub struct LevelResume;

pub fn on_level_resume(
    _: On<LevelResume>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(GameState::Running);
    page_state.set(MenuPage::Overlay);
}

#[derive(Event)]
pub struct LevelRestart;

pub fn on_level_restart(
    _: On<LevelRestart>,
    mut commands: Commands,
    entities_q: Query<Entity, With<DespawnOnExit<MainState>>>,
    mut main_state: ResMut<NextState<MainState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    for entity in entities_q {
        commands.entity(entity).despawn();
    }
    main_state.set(MainState::Game);
    game_state.set(GameState::Running);
    page_state.set(MenuPage::Overlay);
}

#[derive(Event)]
pub struct LevelComplete;

pub fn on_level_complete(
    _: On<LevelComplete>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(GameState::Stopped);
    page_state.set(MenuPage::LevelComplete);
}
