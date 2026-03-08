use crate::components::*;
use crate::events::TogglePausedState;
use crate::states::{GameState, PausedState};
use crate::ui::GameUI;
use bevy::prelude::*;

mod components;
mod events;
mod states;
mod ui;

pub const SPEED_MULTIPLIER: f32 = 200.;

fn main() {
    let mut app = App::new();

    app.add_plugins((DefaultPlugins, GameUI))
        .init_state::<GameState>()
        .add_sub_state::<PausedState>()
        .add_observer(on_toggle_pause_state)
        .add_systems(Startup, spawn_camera)
        .add_systems(OnEnter(GameState::Playing), spawn_playable_plane)
        .add_systems(
            Update,
            (
                control_playable_plane.run_if(in_state(PausedState::Playing)),
                watching_input_to_pause_or_resume.run_if(in_state(GameState::Playing)),
            ),
        );

    app.add_plugins((
        bevy_inspector_egui::bevy_egui::EguiPlugin::default(),
        bevy_inspector_egui::quick::WorldInspectorPlugin::new(),
    ));

    app.run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(MyCamera);
}

pub fn spawn_playable_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<State<GameState>>,
) {
    commands.spawn((
        DespawnOnExit(game_state.clone()),
        PlayablePlane,
        Mesh2d(meshes.add(Rectangle::new(100., 20.))),
        MeshMaterial2d(materials.add(Color::from(bevy::color::palettes::basic::BLACK))),
        Transform::default(),
    ));
}

pub fn control_playable_plane(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        transform.translation.x = transform.translation.x + SPEED_MULTIPLIER * time.delta_secs();
    } else if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        transform.translation.x = transform.translation.x - SPEED_MULTIPLIER * time.delta_secs();
    }
}

pub fn watching_input_to_pause_or_resume(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        commands.trigger(TogglePausedState);
    }
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
