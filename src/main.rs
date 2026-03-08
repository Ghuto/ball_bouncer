use crate::states::{GameState, PausedState};
use crate::ui::GameUI;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use systems::*;

mod components;
mod events;
mod states;
mod systems;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, GameUI))
        .init_state::<GameState>()
        .add_sub_state::<PausedState>()
        .add_observer(on_toggle_pause_state)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(GameState::Playing),
            (spawn_playable_plane, spawn_ball).chain(),
        )
        .add_systems(
            Update,
            (
                (
                    (control_playable_plane, contain_plane_in_screen).chain(),
                    move_ball,
                )
                    .run_if(in_state(PausedState::Playing)),
                watching_input_to_pause_or_resume
                    .run_if(in_state(GameState::Playing).and(input_just_pressed(KeyCode::Escape))),
            ),
        );

    app.add_plugins((
        bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
        bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ));

    app.run();
}
