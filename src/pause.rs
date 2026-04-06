use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameState, ui_pages::MenuPage};

#[derive(Event)]
pub struct PauseGame;

#[derive(Event)]
pub struct ResumeGame;

pub fn watch_input_for_pause(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Escape) {
        commands.trigger(PauseGame);
    }
}

pub fn pause_game(
    _: On<PauseGame>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
    entity_q: Query<Entity, With<RigidBody>>,
) {
    game_state.set(GameState::Paused);
    page_state.set(MenuPage::Pause);
    for entity in entity_q {
        commands.entity(entity).insert(RigidBodyDisabled);
    }
}

pub fn resume_game(
    _: On<ResumeGame>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
    entity_q: Query<Entity, (With<RigidBody>, With<RigidBodyDisabled>)>,
) {
    game_state.set(GameState::Running);
    page_state.set(MenuPage::OverLay);
    for entity in entity_q {
        commands.entity(entity).remove::<RigidBodyDisabled>();
    }
}
