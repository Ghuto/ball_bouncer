use avian2d::prelude::{RigidBody, RigidBodyDisabled};
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
    Running,
    Stopped,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainState>()
            .add_sub_state::<GameState>()
            .add_systems(
                OnTransition {
                    exited: GameState::Stopped,
                    entered: GameState::Running,
                },
                resume,
            )
            .add_systems(
                OnTransition {
                    exited: GameState::Running,
                    entered: GameState::Stopped,
                },
                pause,
            );
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
