use std::fmt::Debug;

use crate::game_state::MainState;
use crate::general_events::*;
use crate::sounds::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub const BORDER_COLOR: Color = Color::Srgba(tailwind::GRAY_700);
pub const TEXT_COLOR: Color = Color::Srgba(tailwind::SLATE_400);
pub const TEXT_HOVER_COLOR: Color = Color::Srgba(tailwind::SLATE_100);
pub const BACKGROUND_COLOR: Color = Color::Srgba(tailwind::ZINC_800);

pub const INFO_COLOR: Color = Color::Srgba(tailwind::YELLOW_200);
pub const DANGER_COLOR: Color = Color::Srgba(tailwind::RED_500);
pub const SUCCESS_COLOR: Color = Color::Srgba(tailwind::GREEN_500);

pub mod page_level_completed;
pub mod page_level_failed;
pub mod page_level_paused;
pub mod page_level_select;
pub mod page_overlay;
pub mod page_title;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MenuPage {
    #[default]
    Title,
    Overlay,
    LevelFailed,
    LevelPaused,
    LevelSelect,
    LevelComplete,
}

fn on_click_restart_button(_trigger: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(LevelRestart);
}

fn on_click_go_to_main_menu_button(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MainState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(MainState::Title);
    page_state.set(MenuPage::Title);
}

fn on_hover_play_sound(_: On<Pointer<Over>>, mut command: Commands) {
    command.trigger(PlaySoundEffect(SoundEffect::MenuHover));
}

fn on_event_update_ui_entity<Event: Debug + Clone + Reflect>(
    new_text_color: Color,
) -> impl Fn(On<Pointer<Event>>, Query<&Children>, Query<&mut TextColor>) {
    move |trigger, mut button_q, mut text_color_q| {
        let children = button_q.get_mut(trigger.entity).unwrap();

        for child in children {
            if let Ok(mut text_color) = text_color_q.get_mut(*child) {
                text_color.0 = new_text_color;
            }
        }
    }
}
