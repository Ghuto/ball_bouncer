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
            .add_observer(on_insert_rigid_body_disable.run_if(
                in_state(GameState::Paused).or_eager(in_state(GameState::WaitingForInput)),
            ));
    }
}

/// when RigidBody is inserted disable it by inserting RigidBodyDisabled
fn on_insert_rigid_body_disable(trigger: On<Insert, RigidBody>, mut commands: Commands) {
    commands.entity(trigger.entity).insert(RigidBodyDisabled);
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
