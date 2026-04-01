use avian2d::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

use crate::ball::*;
use crate::border::*;
use crate::camera::*;
use crate::playable_plane::*;
use crate::states::*;
use crate::ui::*;

mod ball;
mod border;
mod camera;
mod playable_plane;
mod states;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, GameUI, PhysicsPlugins::default()))
        .init_state::<GameState>()
        .add_sub_state::<PausedState>()
        .add_observer(on_toggle_pause_state)
        .add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(GameState::Playing),
            (spawn_playable_plane, spawn_ball, spawn_border).chain(),
        )
        .add_systems(
            Update,
            (
                (control_playable_plane,).run_if(in_state(PausedState::Playing)),
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
