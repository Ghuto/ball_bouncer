use bevy::prelude::*;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::LevelSelect),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Start,
                ..Default::default()
            },
            BackgroundColor(BACKGROUND_COLOR),
        ))
        .with_children(|container| {
            container
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        height: Val::Percent(100.),
                        width: Val::Px(250.),
                        border: UiRect::right(Val::Px(5.)),
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_COLOR),
                ))
                .with_children(|content_content| {
                    content_content
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                padding: UiRect::left(Val::Px(10.)),
                                ..Default::default()
                            },
                            Button,
                            children![(
                                Text::new("level 1"),
                                TextColor(TEXT_COLOR),
                                TextFont::from_font_size(36.),
                                Pickable::IGNORE,
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                        .observe(on_hover_play_sound);

                    content_content
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                padding: UiRect::left(Val::Px(10.)),
                                margin: UiRect::top(Val::Auto),
                                ..Default::default()
                            },
                            Button,
                            children![(
                                Text::new("Return"),
                                TextColor(TEXT_COLOR),
                                TextFont::from_font_size(36.),
                                Pickable::IGNORE,
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                        .observe(on_hover_play_sound)
                        .observe(on_click_return);
                });

            container
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Start,
                        height: Val::Percent(100.),
                        width: Val::Px(250.),
                        border: UiRect::right(Val::Px(5.)),
                        ..Default::default()
                    },
                    BorderColor::all(BORDER_COLOR),
                ))
                .with_children(|content_content| {
                    content_content.spawn((
                        Node {
                            width: Val::Percent(100.),
                            padding: UiRect::left(Val::Px(10.)),
                            ..Default::default()
                        },
                        Button,
                        children![(
                            Text::new("Score:"),
                            TextColor(TEXT_COLOR),
                            TextFont::from_font_size(36.),
                            Pickable::IGNORE,
                        )],
                    ));

                    content_content
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
                                Pickable::IGNORE,
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                        .observe(on_hover_play_sound)
                        .observe(on_click_play_button);
                });
        });
}

fn on_click_return(_trigger: On<Pointer<Click>>, mut page_state: ResMut<NextState<MenuPage>>) {
    page_state.set(MenuPage::Title);
}

fn on_click_play_button(
    _trigger: On<Pointer<Click>>,
    mut game_state: ResMut<NextState<MainState>>,
    mut page_state: ResMut<NextState<MenuPage>>,
) {
    game_state.set(MainState::Game);
    page_state.set(MenuPage::Overlay);
}
