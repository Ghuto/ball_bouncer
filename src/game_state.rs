use avian2d::prelude::{RigidBody, RigidBodyDisabled};
use bevy::prelude::*;

use crate::{general_events::LevelResume, ui_pages::MenuPage};

// main state at which the game starts with title
// and all gameplay functionalities are not running
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MainState {
    #[default]
    Title,
    Game,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect, Event)]
#[source(MainState = MainState::Game)]
pub enum GameState {
    #[default]
    WaitingForInputToBegin,
    Running,
    Stopped,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainState>()
            .add_sub_state::<GameState>()
            .add_systems(OnEnter(GameState::Running), resume)
            .add_systems(OnEnter(GameState::Stopped), pause)
            .add_systems(
                Update,
                press_any_button_to_begin.run_if(in_state(GameState::WaitingForInputToBegin)),
            );
    }
}

/// When Space button is pressed it resumes the level. Intially it is paused
fn press_any_button_to_begin(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Space) {
        commands.trigger(LevelResume);
    }
}

fn pause(mut commands: Commands, rigid_body_q: Query<Entity, With<RigidBody>>) {
    for entity in rigid_body_q {
        commands.entity(entity).insert(RigidBodyDisabled);
    }
}

fn resume(mut commands: Commands, rigid_body_disabled_q: Query<Entity, With<RigidBodyDisabled>>) {
    for entity in rigid_body_disabled_q {
        commands.entity(entity).remove::<RigidBodyDisabled>();
    }
}

pub fn watch_input_for_pause(
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Stopped);
        page_state.set(MenuPage::LevelPaused);
    }
}
