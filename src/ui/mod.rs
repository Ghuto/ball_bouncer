use bevy::prelude::*;
use states::*;

use crate::states::GameState;

mod pages;
mod states;

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuPage>()
            .add_systems(OnEnter(MenuPage::Main), pages::main_menu::build);
    }
}

pub fn on_click_play_button(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Playing);
}

pub fn on_click_quit_button(
    _trigger: On<Pointer<Click>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    app_exit_message_writer.write(AppExit::default());
}

fn update_text_color_on_hover(
    hover_colour: Color,
) -> impl Fn(On<Pointer<Over>>, Query<&Children>, Query<&mut TextColor>) {
    move |trigger, children_q, mut text_color_q| {
        let children = children_q.get(trigger.entity).unwrap();

        for child in children {
            if let Ok(mut text_color) = text_color_q.get_mut(*child) {
                text_color.0 = hover_colour;
            }
        }
    }
}

fn update_text_color_on_hover_end(
    base_colour: Color,
) -> impl Fn(On<Pointer<Out>>, Query<&Children>, Query<&mut TextColor>) {
    move |trigger, children_q, mut text_color_q| {
        let children = children_q.get(trigger.entity).unwrap();

        for child in children {
            if let Ok(mut text_color) = text_color_q.get_mut(*child) {
                text_color.0 = base_colour;
            }
        }
    }
}

