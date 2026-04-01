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

#[derive(Event)]
pub struct TogglePausedState;

pub fn watching_input_to_pause_or_resume(mut commands: Commands) {
    commands.trigger(TogglePausedState);
}

pub fn on_toggle_pause_state(
    _: On<TogglePausedState>,
    mut pause_state_writer: ResMut<NextState<PausedState>>,
    pause_state: Res<State<PausedState>>,
) {
    match pause_state.get() {
        PausedState::Playing => pause_state_writer.set(PausedState::Paused),
        PausedState::Paused => pause_state_writer.set(PausedState::Playing),
    }
}
