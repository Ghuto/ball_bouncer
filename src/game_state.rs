use avian2d::prelude::{Physics, PhysicsTime};
use bevy::prelude::*;

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
    WaitingForInput,
    Running,
    Paused,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainState>()
            .add_sub_state::<GameState>()
            .add_systems(OnEnter(GameState::Running), resume)
            .add_systems(OnEnter(GameState::Paused), pause)
            .add_systems(OnEnter(GameState::WaitingForInput), pause);
    }
}

fn pause(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn resume(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
