use std::fmt::Debug;

use bevy::{color::palettes::tailwind, prelude::*};

use crate::MainState;

mod levels;
mod lost;
mod main;
mod overlay;
mod pause;

pub const BORDER_COLOR: Color = Color::Srgba(tailwind::GRAY_700);
pub const BORDER_HOVER_COLOR: Color = Color::Srgba(tailwind::GRAY_400);
pub const TEXT_COLOR: Color = Color::Srgba(tailwind::SLATE_400);
pub const TEXT_HOVER_COLOR: Color = Color::Srgba(tailwind::SLATE_100);
pub const BACKGROUND_COLOR: Color = Color::Srgba(tailwind::ZINC_800);
pub const INFO_COLOR: Color = Color::Srgba(tailwind::YELLOW_200);
pub const DANGER_COLOR: Color = Color::Srgba(tailwind::RED_500);

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuPage>()
            .add_systems(OnEnter(MenuPage::Main), main::build)
            .add_systems(OnEnter(MenuPage::Lose), lost::build)
            .add_systems(OnEnter(MenuPage::Pause), pause::build)
            .add_systems(OnEnter(MenuPage::OverLay), overlay::build)
            .add_systems(OnEnter(MenuPage::Levels), levels::build);
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum MenuPage {
    #[default]
    Main,
    OverLay,
    Lose,
    Pause,
    Levels,
}

fn on_event_update_ui_entity<Event: Debug + Clone + Reflect>(
    new_text_color: Color,
    optional_new_border_color: Option<Color>,
) -> impl Fn(On<Pointer<Event>>, Query<(&Children, Option<&mut BorderColor>)>, Query<&mut TextColor>) {
    move |trigger, mut button_q, mut text_color_q| {

        let (children, optional_border_color) = button_q.get_mut(trigger.entity).unwrap();

        if let (Some(new_border_color,),Some(mut border_color)) = (optional_new_border_color,optional_border_color) {
            border_color.set_all(new_border_color);
        }

        for child in children {
            if let Ok(mut text_color) = text_color_q.get_mut(*child) {
                text_color.0 = new_text_color;
            }
        }
    }
}