use bevy::prelude::*;

use crate::{
    game_state::{GameState, MainState},
    level::{Level, StartLevel},
};

use super::*;

pub fn menu_button(text: &'static str, font_size: f32) -> impl Scene {
    bsn! {
        Button
        Children[(
            Text::new(text)
            TextColor(TEXT_COLOR)
            TextFont { font_size: px(font_size)}
            Pickable::IGNORE)
        ]
        // play a sound
        on(|_: On<Pointer<Over>>, mut command: Commands|{
            command.trigger(PlaySoundEffect(SoundEffect::MenuHover))
        })
        // on hover change text color
        on(| trigger: On<Pointer<Over>>, button_q: Query<&Children>, mut text_color_q: Query<&mut TextColor> |{
            let children = button_q.get(trigger.entity).unwrap();

            for child in children {
                if let Ok(mut text_color) = text_color_q.get_mut(*child) {
                    text_color.0 = TEXT_HOVER_COLOR;
                }
            }
        })
        // on over change text color back
        on(| trigger: On<Pointer<Out>>, button_q: Query<&Children>, mut text_color_q: Query<&mut TextColor> |{
            let children = button_q.get(trigger.entity).unwrap();

            for child in children {
                if let Ok(mut text_color) = text_color_q.get_mut(*child) {
                    text_color.0 = TEXT_COLOR;
                }
            }
        })
    }
}

pub fn resume_button() -> impl Scene {
    bsn! {
        Node {
            justify_content: JustifyContent::Center,
            margin: UiRect::vertical(Val::Px(5.)),
            width: Val::Percent(100.),
        }
        menu_button("Resume",30.)
        on(|_trigger: On<Pointer<Click>>, mut commands: Commands|{
            commands.trigger(LevelResume)
        })
    }
}

pub fn restart_button() -> impl Scene {
    bsn! {
        Node {
            justify_content: JustifyContent::Center,
            margin: UiRect::vertical(Val::Px(5.)),
            width: Val::Percent(100.),
        }
        menu_button("Restart",30.)
        on(|_trigger: On<Pointer<Click>>, mut commands: Commands,mut page_state: ResMut<NextState<MenuPage>>|{
            page_state.set(MenuPage::InputToBeginLevel);
            commands.trigger(LevelRestart);
        })
    }
}

pub fn main_menu_button() -> impl Scene {
    bsn! {
        Node {
            justify_content: JustifyContent::Center,
            margin: UiRect::vertical(Val::Px(5.)),
            width: Val::Percent(100.),
        }
        menu_button("Main Menu",30.)
        on(|_trigger: On<Pointer<Click>>, mut game_state: ResMut<NextState<MainState>>, mut page_state: ResMut<NextState<MenuPage>>,|{
            game_state.set(MainState::Title);
            page_state.set(MenuPage::Title);
        })
    }
}

pub fn level_selection_button() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.),
            padding: UiRect::left(Val::Px(10.)),
        }
        menu_button("Level Selection",30.)
        on(|_trigger: On<Pointer<Click>>, mut page_state_set: ResMut<NextState<MenuPage>>,|{
            page_state_set.set(MenuPage::LevelSelection);
        })
    }
}

pub fn quit_button() -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.),
            padding: UiRect::left(Val::Px(10.)),
        }
        menu_button("Quit",30.)
        on(|_trigger: On<Pointer<Click>>, mut app_exit_message_writer: MessageWriter<AppExit>,|{
            app_exit_message_writer.write(AppExit::default());
        })
    }
}

pub fn return_button() -> impl Scene {
    bsn! {
        menu_button("Return",30.)
        Node {
            width: Val::Percent(100.),
            padding: UiRect::left(Val::Px(10.)),
            margin: UiRect::top(Val::Auto),
        }
        on(|_trigger: On<Pointer<Click>>, mut page_state: ResMut<NextState<MenuPage>>|{
            page_state.set(MenuPage::Title);
        })
    }
}

pub fn level_button(level: Level) -> impl Scene {
    bsn! {
        Node {
            width: Val::Percent(100.),
            padding: UiRect::left(Val::Px(10.)),
        }
        menu_button(level.get_label(), 30.)
        on(move |_trigger: On<Pointer<Click>>, mut commands: Commands,mut page_state: ResMut<NextState<MenuPage>>| {
            page_state.set(MenuPage::InputToBeginLevel);
            commands.trigger(StartLevel(level));
        })
    }
}

/// uses "||" as an pause icon
pub fn pause_icon_button() -> impl Scene {
    bsn! {
        Node {
            // override default AlignItems::Stretch
            align_self: AlignSelf::Start,
        }
        // using "||" to imitate pause icon
        menu_button("||", 30.)
        on(|_trigger: On<Pointer<Click>>, mut game_state: ResMut<NextState<GameState>>,mut page_state: ResMut<NextState<MenuPage>>| {
            page_state.set(MenuPage::LevelPaused);
            game_state.set(GameState::Stopped);
        })
    }
}
