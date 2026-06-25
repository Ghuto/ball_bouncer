use crate::general_events::*;
use crate::sounds::*;
use crate::ui_pages::buttons::*;
use crate::ui_pages::templates::*;
use bevy::color::palettes::tailwind;
use bevy::prelude::*;

pub const BORDER_COLOR: Color = Color::Srgba(tailwind::GRAY_700);
pub const TEXT_COLOR: Color = Color::Srgba(tailwind::SLATE_400);
pub const TEXT_HOVER_COLOR: Color = Color::Srgba(tailwind::SLATE_100);

pub const INFO_COLOR: Color = Color::Srgba(tailwind::YELLOW_200);
pub const DANGER_COLOR: Color = Color::Srgba(tailwind::RED_500);
pub const SUCCESS_COLOR: Color = Color::Srgba(tailwind::GREEN_500);

pub const fn background_color(alpha: f32) -> Color {
    Color::Srgba(Srgba::new(
        tailwind::ZINC_800.red,
        tailwind::ZINC_800.green,
        tailwind::ZINC_800.blue,
        alpha,
    ))
}

pub mod buttons;
pub mod templates;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuPage>()
            .add_systems(OnEnter(MenuPage::Title), title_page.spawn())
            .add_systems(OnEnter(MenuPage::Overlay), overlay_page.spawn())
            .add_systems(
                OnEnter(MenuPage::LevelSelection),
                level_selection_page.spawn(),
            )
            .add_systems(OnEnter(MenuPage::LevelFailed), build)
            .add_systems(OnEnter(MenuPage::LevelPaused), build)
            .add_systems(OnEnter(MenuPage::LevelComplete), build)
            .add_systems(OnEnter(MenuPage::InputToBeginLevel), build);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MenuPage {
    #[default]
    Title,
    Overlay,
    LevelFailed,
    LevelPaused,
    LevelSelection,
    LevelComplete,
    //press Space button to begin
    InputToBeginLevel,
}

/// a workaround because spawning scenes with parameters in add_systems is not possible
pub fn build(mut commands: Commands, menu_page_state: Res<State<MenuPage>>) {
    match menu_page_state.get() {
        MenuPage::LevelFailed => commands.spawn_scene(level_status_page(
            "Game Over",
            DANGER_COLOR,
            bsn_list![restart_button(), main_menu_button(),],
        )),
        MenuPage::LevelPaused => commands.spawn_scene(level_status_page(
            "Paused",
            INFO_COLOR,
            bsn_list![resume_button(), restart_button(), main_menu_button(),],
        )),
        MenuPage::LevelComplete => commands.spawn_scene(level_status_page(
            "Level Complete",
            SUCCESS_COLOR,
            bsn_list![restart_button(), main_menu_button(),],
        )),
        MenuPage::InputToBeginLevel => commands.spawn_scene(level_status_page(
            "Press 'Space Bar' to begin",
            TEXT_COLOR,
            bsn_list![],
        )),
        _ => return,
    };
}
