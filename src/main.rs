use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::ball::*;
use crate::brick::*;
use crate::camera::*;
use crate::controllable_plane::*;
#[cfg(feature = "editor")]
use crate::editor::EditorPlugin;
use crate::game_state::*;
use crate::general_events::*;
use crate::level::*;
use crate::sounds::*;
use crate::ui_pages::MenuPage;
use crate::ui_pages::UIPlugin;

mod ball;
mod brick;
mod camera;
mod controllable_plane;
#[cfg(feature = "editor")]
mod editor;
mod game_state;
mod general_events;
mod level;
mod level_border;
mod powerup;
mod sounds;
mod ui_pages;
#[cfg(feature = "inspector")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

const DEFAULT_WIDTH_RESOLUTION: u32 = 1280;
const DEFAULT_HEIGHT_RESOLUTION: u32 = 720;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default, // the default layer that objects are assigned to
    ControllablePlane,
    PickUp,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    resolution: WindowResolution::new(
                        DEFAULT_WIDTH_RESOLUTION,
                        DEFAULT_HEIGHT_RESOLUTION,
                    ),
                    fit_canvas_to_parent: true,
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            PhysicsPlugins::default(),
            StatePlugin,
            UIPlugin,
            #[cfg(feature = "editor")]
            EditorPlugin,
            #[cfg(feature = "inspector")]
            EguiPlugin::default(),
            #[cfg(feature = "inspector")]
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, camera_scene.spawn())
        .add_systems(
            Update,
            (
                (
                    watch_input_for_pause,
                    control_plane,
                    despawn_lost_balls,
                    win_check,
                    lose_check,
                )
                    .run_if(in_state(GameState::Running)),
                press_any_button_to_begin.run_if(in_state(GameState::WaitingForInput)),
            ),
        )
        .init_resource::<BrickMesh>()
        .init_resource::<BallMesh>()
        .init_resource::<ControllablePlaneMesh>()
        .init_resource::<SoundEffects>()
        .add_observer(SoundEffect::on_trigger)
        .add_observer(ResumeLevel::on_trigger)
        .add_observer(RestartLevel::on_trigger)
        .add_observer(Level::on_trigger)
        .add_observer(ModifyPlane::on_trigger)
        .add_observer(ModifyBall::on_trigger)
        .add_observer(Brick::on_insert)
        .run();
}

fn watch_input_for_pause(
    input: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
        page_state.set(MenuPage::LevelPaused);
    }
}

/// When Space button is pressed it resumes the level. Intially it is paused
fn press_any_button_to_begin(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Space) {
        commands.trigger(ResumeLevel);
    }
}

#[derive(Event)]
pub enum CheckCondition {
    Win,
    GameOver,
}

fn win_check(
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
    brick_q: Query<&Brick>,
) {
    if brick_q.is_empty() {
        game_state.set(GameState::Paused);
        page_state.set(MenuPage::LevelComplete);
    }
}

fn lose_check(
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
    ball_q: Query<&Ball>,
) {
    if ball_q.is_empty() {
        game_state.set(GameState::Paused);
        page_state.set(MenuPage::LevelFailed);
    }
}
