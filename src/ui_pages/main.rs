use bevy::prelude::*;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::Main),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                ..Default::default()
            },
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .with_children(|container| {
            container
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        height: Val::Percent(100.),
                        width: Val::Px(250.),
                        border: UiRect::left(Val::Px(5.)),
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_COLOR),
                ))
                .with_children(|right_container| {
                    right_container
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                padding: UiRect::left(Val::Px(10.)),
                                ..Default::default()
                            },
                            Button,
                            children![(
                                Text::new("Play"),
                                TextColor(TEXT_COLOR),
                                TextFont::from_font_size(36.),
                            )],
                        ))
                        .observe(on_click_play_button)
                        .observe(change_color_on_over(TEXT_HOVER_COLOR,None))
                        .observe(change_color_on_out(TEXT_COLOR,None));

                    right_container
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                padding: UiRect::left(Val::Px(10.)),
                                ..Default::default()
                            },
                            Button,
                            children![(
                                Text::new("Settings"),
                                TextColor(TEXT_COLOR),
                                TextFont::from_font_size(36.),
                            )],
                        ))
                        .observe(change_color_on_over(TEXT_HOVER_COLOR,None))
                        .observe(change_color_on_out(TEXT_COLOR,None));

                    right_container
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                padding: UiRect::left(Val::Px(10.)),
                                ..Default::default()
                            },
                            Button,
                            children![(
                                Text::new("Quit"),
                                TextColor(TEXT_COLOR),
                                TextFont::from_font_size(36.),
                            )],
                        ))
                        .observe(on_click_quit_button)
                        .observe(change_color_on_over(TEXT_HOVER_COLOR,None))
                        .observe(change_color_on_out(TEXT_COLOR,None));
                });
        });
}

fn on_click_play_button(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MainState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(MainState::GamePlay);
    page_state.set(MenuPage::OverLay);
}

fn on_click_quit_button(
    _trigger: On<Pointer<Click>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    app_exit_message_writer.write(AppExit::default());
}
