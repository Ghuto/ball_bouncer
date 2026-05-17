use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::WindowResolution;
#[cfg(feature = "inspector")]
use bevy_remote::RemotePlugin;
#[cfg(feature = "inspector")]
use bevy_remote::http::RemoteHttpPlugin;

use crate::ball::*;
use crate::border::*;
use crate::brick::*;
use crate::camera::*;
use crate::controllable_plane::*;
use crate::game_state::*;
use crate::general_events::*;
use crate::sounds::*;
use crate::ui_pages::*;

mod ball;
mod border;
mod brick;
mod camera;
mod controllable_plane;
mod game_state;
mod general_events;
mod sounds;
mod ui_pages;

const DEFAULT_WIDTH_RESOLUTION: u32 = 1280;
const DEFAULT_HEIGHT_RESOLUTION: u32 = 720;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                resolution: WindowResolution::new(
                    DEFAULT_WIDTH_RESOLUTION,
                    DEFAULT_HEIGHT_RESOLUTION,
                ),
                fit_canvas_to_parent: true,
                ..Default::default()
            }),
            ..Default::default()
        }),
        PhysicsPlugins::default(),
        StatePlugin,
    ))
    .add_systems(Startup, (spawn_camera, load_sound_effects))
    .add_systems(
        Update,
        (
            watch_input_for_pause,
            control_plane,
            despawn_lost_balls,
            watch_game_over_condition,
            watch_win_condition,
        )
            .run_if(in_state(GameState::Running)),
    )
    .add_systems(
        OnEnter(MainState::Game),
        (
            trigger_event(SpawnControllablePlane {
                at_position: Vec3::new(0., -250., 0.),
            }),
            trigger_event(SpawnBall {
                at_position: Vec3::new(0., 50., 0.),
            }),
            trigger_event(SpawnBrick {
                at_position: Vec3::new(0., 200., 0.),
            }),
            trigger_event(SpawnBorder),
        ),
    )
    .init_state::<MenuPage>()
    .add_systems(OnEnter(MenuPage::Title), ui_pages::page_title::build)
    .add_systems(OnEnter(MenuPage::Overlay), page_overlay::build)
    .add_systems(OnEnter(MenuPage::LevelFailed), page_level_failed::build)
    .add_systems(OnEnter(MenuPage::LevelPaused), page_level_paused::build)
    .add_systems(OnEnter(MenuPage::LevelSelect), page_level_select::build)
    .add_systems(
        OnEnter(MenuPage::LevelComplete),
        page_level_completed::build,
    )
    .add_observer(spawn_controllable_plane)
    .add_observer(spawn_ball)
    .add_observer(spawn_border)
    .add_observer(on_spawn_brick)
    .add_observer(on_play_sound)
    .add_observer(on_level_failed)
    .add_observer(on_level_pause)
    .add_observer(on_level_resume)
    .add_observer(on_level_restart)
    .add_observer(on_level_complete);

    #[cfg(feature = "inspector")]
    app.add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default());

    app.run();
}

fn trigger_event<'a>(event: impl Event<Trigger<'a>: Default> + Clone) -> impl Fn(Commands) {
    move |mut commands| {
        let event = event.clone();
        commands.trigger(event);
    }
}

pub fn watch_input_for_pause(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Escape) {
        commands.trigger(LevelPause);
    }
}

pub fn watch_game_over_condition(mut commands: Commands, ball_q: Query<&Ball>) {
    if ball_q.is_empty() {
        commands.trigger(LevelFailed);
    }
}

pub fn watch_win_condition(mut commands: Commands, brick_q: Query<&Brick>) {
    if brick_q.is_empty() {
        commands.trigger(LevelComplete);
    }
}
