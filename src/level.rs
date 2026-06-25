use bevy::prelude::*;

use crate::{
    ball::SpawnBall, border::SpawnBorder, controllable_plane::SpawnControllablePlane,
    game_state::MainState, ui_pages::buttons::level_button,
};

#[derive(Component)]
pub struct LevelRoot;

#[derive(Resource,Reflect)]
#[reflect(Resource)]
pub struct InLevel(pub Level);

#[derive(Clone, Copy,Reflect)]
pub enum Level {
    First,
    Second,
}

impl Level {
    pub fn get_file_name(&self) -> &'static str {
        match self {
            Level::First => "levels/level1.ron",
            Level::Second => "levels/level2.ron",
        }
    }
    pub fn get_label(&self) -> &'static str {
        match self {
            Level::First => "Level 1",
            Level::Second => "Level 2",
        }
    }

    pub fn get_list_of_level_buttons() -> impl SceneList {
        bsn_list![level_button(Level::First),level_button(Level::Second)]
    }

}



pub fn remove_level_resource(mut commands: Commands) {
    commands.remove_resource::<InLevel>();
}

#[derive(Event)]
pub struct StartLevel(pub Level);

pub fn start_level(
    start_level: On<StartLevel>,
    mut commands: Commands,
    mut main_state: ResMut<NextState<MainState>>,
) {
    main_state.set(MainState::Game);
    commands.insert_resource(InLevel(start_level.0));
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    in_level: Res<InLevel>
) {

    commands.trigger(SpawnBorder);
    commands.trigger(SpawnBall {
        at_position: Vec3::new(0., 50., 0.),
    });
    commands.trigger(SpawnControllablePlane {
        at_position: Vec3::new(0., -250., 0.),
    });
    commands.spawn((
        DynamicWorldRoot(asset_server.load(in_level.0.get_file_name())),
        DespawnOnExit::<MainState>(MainState::Game),
        LevelRoot,
    ));
}
