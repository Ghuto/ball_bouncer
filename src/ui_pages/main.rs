use bevy::prelude::*;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::Main),
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
                                Text::new("Levels"),
                                TextColor(TEXT_COLOR),
                                TextFont::from_font_size(36.),
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR, None))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR, None))
                        .observe(on_click_levels);

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
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR, None))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR, None));

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
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR, None))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR, None))
                        .observe(on_click_quit_button);
                });
        });
}

fn on_click_levels(_trigger: On<Pointer<Click>>, mut page_state_set: ResMut<NextState<MenuPage>>) {
    page_state_set.set(MenuPage::Levels);
}

fn on_click_quit_button(
    _trigger: On<Pointer<Click>>,
    mut app_exit_message_writer: MessageWriter<AppExit>,
) {
    app_exit_message_writer.write(AppExit::default());
}
