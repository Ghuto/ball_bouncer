use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    ball::Ball, controllable_plane::ControllablePlane, game_state::MainState,
    level_border::LevelBorder, ui_pages::buttons::level_button,
};

// This is the relationship FROM child TO parent
#[derive(Component)]
#[relationship(relationship_target = LevelEntities)]
struct InLevel(Entity);

// This is the relationship target on the parent
#[derive(Component)]
#[relationship_target(relationship = InLevel)]
struct LevelEntities(Vec<Entity>);

#[derive(Component, Clone, Reflect, Event, Debug, Eq, PartialEq, Hash)]
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
        bsn_list![level_button(Level::First), level_button(Level::Second)]
    }

    pub fn on_trigger(
        start_level: On<Self>,
        mut commands: Commands,
        mut main_state: ResMut<NextState<MainState>>,
        window: Single<&Window, With<PrimaryWindow>>,
    ) {
        let level = start_level.event().clone();
        let level_path = level.get_file_name();

        main_state.set(MainState::Game);

        commands.queue_spawn_scene({
            bsn! {
                DespawnOnExit::<MainState>(MainState::Game)
                template(move |_|{
                    Ok(level.clone())
                })
                LevelEntities[
                    @Ball,
                    @LevelBorder{
                        @border_vertices: LevelBorder::vertices_from_window(&window)
                    },
                    @ControllablePlane,
                    DynamicWorldRoot(level_path)
                ]
            }
        });
    }
}
