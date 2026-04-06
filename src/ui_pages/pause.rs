use bevy::prelude::*;

use crate::pause::ResumeGame;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::Pause),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            BackgroundColor(BACKGROUND_COLOR.with_alpha(0.1)),
            Pickable::IGNORE,
        ))
        .with_children(|container| {
            container
                .spawn((Node {
                    height: Val::Percent(40.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },))
                .with_children(|top_container| {
                    top_container.spawn((
                        Text::new("Paused"),
                        TextFont::default().with_font_size(60.),
                        TextColor(INFO_COLOR),
                    ));
                });

            container
                .spawn((Node {
                    height: Val::Percent(40.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..Default::default()
                },))
                .with_children(|top_container| {
                    top_container
                        .spawn((
                            Button,
                            BorderColor::all(BORDER_COLOR),
                            Node {
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(5.)),
                                border_radius: BorderRadius::all(Val::Px(20.)),
                                margin: UiRect::all(Val::Percent(10.)),
                                width: Val::Percent(30.),
                                ..Default::default()
                            },
                        ))
                        .with_children(|button_container| {
                            button_container.spawn((
                                Text::new("Go To Main Menu"),
                                TextColor(TEXT_COLOR),
                                TextFont::default().with_font_size(30.),
                            ));
                        })
                        .observe(change_color_on_over(
                            TEXT_HOVER_COLOR,
                            Some(BORDER_HOVER_COLOR),
                        ))
                        .observe(change_color_on_out(TEXT_COLOR, Some(BORDER_COLOR)))
                        .observe(on_click_go_to_main_menu_button);

                    top_container
                        .spawn((
                            Button,
                            BorderColor::all(BORDER_COLOR),
                            Node {
                                justify_content: JustifyContent::Center,
                                border: UiRect::all(Val::Px(5.)),
                                border_radius: BorderRadius::all(Val::Px(20.)),
                                margin: UiRect::all(Val::Percent(10.)),
                                width: Val::Percent(30.),
                                ..Default::default()
                            },
                        ))
                        .with_children(|button_container| {
                            button_container.spawn((
                                Text::new("Resume"),
                                TextColor(TEXT_COLOR),
                                TextFont::default().with_font_size(30.),
                            ));
                        })
                        .observe(change_color_on_over(
                            TEXT_HOVER_COLOR,
                            Some(BORDER_HOVER_COLOR),
                        ))
                        .observe(change_color_on_out(TEXT_COLOR, Some(BORDER_COLOR)))
                        .observe(on_click_resume_button);
                });
        });
}

fn on_click_resume_button(_trigger: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(ResumeGame);
}

fn on_click_go_to_main_menu_button(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MainState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(MainState::TitleMenu);
    page_state.set(MenuPage::Main);
}
