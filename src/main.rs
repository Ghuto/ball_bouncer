use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;

use crate::ball::*;
use crate::border::*;
use crate::brick::*;
use crate::camera::*;
use crate::controllable_plane::*;
#[cfg(feature = "editor")]
use crate::editor::EditorPlugin;
use crate::game_state::*;
use crate::general_events::*;
use crate::level::*;
use crate::powerup::*;
use crate::sounds::*;
use crate::ui_pages::UIPlugin;

mod ball;
mod border;
mod brick;
mod camera;
mod controllable_plane;
#[cfg(feature = "editor")]
mod editor;
mod game_state;
mod general_events;
mod level;
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
    Brick,
    Ball,
    Border,
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
        .add_systems(Startup, (spawn_camera, load_sound_effects))
        .add_systems(
            FixedUpdate,
            despawn_lost_balls.run_if(in_state(GameState::Running)),
        )
        .add_systems(
            Update,
            ((watch_input_for_pause, control_plane).run_if(in_state(GameState::Running)),),
        )
        .add_systems(
            OnTransition {
                exited: MainState::Game,
                entered: MainState::Title,
            },
            remove_level_resource,
        )
        .add_systems(OnEnter(GameState::WaitingForInputToBegin), spawn_level)
        .add_observer(spawn_controllable_plane)
        .add_observer(spawn_ball)
        .add_observer(spawn_border)
        .add_observer(on_spawn_brick)
        .add_observer(on_play_sound)
        .add_observer(on_level_resume)
        .add_observer(on_level_restart)
        .add_observer(on_level_complete)
        .add_observer(check_win_condition)
        .add_observer(check_game_over_condition)
        .add_observer(level_selected)
        .init_resource::<BrickMesh>()
        .add_observer(on_brick_insert)
        .init_resource::<BallMesh>()
        .init_resource::<ControllablePlaneMesh>()
        .add_observer(try_to_spawn_power_up_pick_up)
        .add_observer(modify_plane)
        .add_observer(modify_ball)
        .run();
}
