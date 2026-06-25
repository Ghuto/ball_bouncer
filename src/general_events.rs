use bevy::prelude::*;

use crate::{
    InLevel, ball::Ball, brick::Brick, game_state::GameState, level::StartLevel, ui_pages::MenuPage,
};

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
    mut game_state: ResMut<NextState<GameState>>,
    in_level: Res<InLevel>,
) {
    commands.trigger(StartLevel(in_level.0));
    game_state.set(GameState::WaitingForInputToBegin);
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

#[derive(Event)]
pub struct CheckGameOverCondition;

pub fn check_game_over_condition(
    _: On<CheckGameOverCondition>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
    ball_q: Query<&Ball>,
) {
    if ball_q.is_empty() {
        game_state.set(GameState::Stopped);
        page_state.set(MenuPage::LevelFailed);
    }
}

#[derive(Event)]
pub struct CheckWinCondition;

pub fn check_win_condition(
    _: On<CheckWinCondition>,
    mut commands: Commands,
    brick_q: Query<&Brick>,
) {
    if brick_q.is_empty() {
        commands.trigger(LevelComplete);
    }
}
