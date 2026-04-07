use avian2d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "inspector")]
use bevy_remote::RemotePlugin;
#[cfg(feature = "inspector")]
use bevy_remote::http::RemoteHttpPlugin;

use crate::ball::*;
use crate::border::*;
use crate::camera::*;
use crate::pause::*;
use crate::playable_plane::*;
use crate::ui_pages::*;

mod ball;
mod border;
mod camera;
mod pause;
mod playable_plane;
mod ui_pages;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }),
        PhysicsPlugins::default(),
        GameUI,
    ))
    .init_state::<MainState>()
    .add_sub_state::<GameState>()
    .add_systems(Startup, spawn_camera)
    .add_systems(
        OnEnter(MainState::GamePlay),
        (
            trigger_event(SpawnPlayablePlane {
                at_position: Vec3::new(0., 0., 0.),
            }),
            trigger_event(SpawnBall {
                at_position: Vec3::new(0., 50., 0.),
            }),
            trigger_event(SpawnBorder),
        ),
    )
    .add_systems(
        Update,
        ((
            watch_input_for_pause,
            control_playable_plane,
            watch_game_over_condition,
        )
            .run_if(in_state(GameState::Running)),),
    )
    .add_observer(pause_game)
    .add_observer(resume_game)
    .add_observer(spawn_playable_plane)
    .add_observer(spawn_ball)
    .add_observer(spawn_border)
    .add_observer(on_game_over)
    .add_observer(on_restart);

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

// main state at which the game starts with title
// and all gameplay functionalities are not running
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MainState {
    #[default]
    TitleMenu,
    GamePlay,
}

#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[source(MainState = MainState::GamePlay)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}

#[derive(Event)]
pub struct RestartGame;

pub fn on_restart(
    _: On<RestartGame>,
    mut commands: Commands,
    entities_q: Query<Entity, Or<(With<PlayablePlane>, With<Ball>)>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    for entity in entities_q {
        commands.entity(entity).despawn();
    }
    commands.trigger(SpawnPlayablePlane {
        at_position: Vec3::new(0., 0., 0.),
    });
    commands.trigger(SpawnBall {
        at_position: Vec3::new(0., 50., 0.),
    });
    commands.trigger(SpawnBorder);
    game_state.set(GameState::Running);
    page_state.set(MenuPage::OverLay);
}

#[derive(Event)]
pub struct GameOver;

pub fn on_game_over(
    _: On<GameOver>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
    entity_q: Query<Entity, With<RigidBody>>,
) {
    game_state.set(GameState::Paused);
    page_state.set(MenuPage::Lose);
    for entity in entity_q {
        commands.entity(entity).insert(RigidBodyDisabled);
    }
}
