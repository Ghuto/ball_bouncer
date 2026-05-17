use bevy::prelude::*;

use crate::LevelResume;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::LevelPaused),
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
            container.spawn((
                Node {
                    height: Val::Percent(40.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                children![(
                    Text::new("Paused"),
                    TextFont::default().with_font_size(60.),
                    TextColor(INFO_COLOR),
                    Pickable::IGNORE,
                )],
            ));

            container
                .spawn((Node {
                    height: Val::Percent(40.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },))
                .with_children(|top_container| {
                    top_container
                        .spawn((
                            Button,
                            Node {
                                justify_content: JustifyContent::Center,
                                margin: UiRect::vertical(Val::Px(5.)),
                                width: Val::Percent(100.),
                                ..Default::default()
                            },
                            children![(
                                Text::new("Resume"),
                                TextColor(TEXT_COLOR),
                                TextFont::default().with_font_size(30.),
                                Pickable::IGNORE,
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                        .observe(on_hover_play_sound)
                        .observe(on_click_resume_button);

                    top_container
                        .spawn((
                            Button,
                            Node {
                                justify_content: JustifyContent::Center,
                                margin: UiRect::vertical(Val::Px(5.)),
                                width: Val::Percent(100.),
                                ..Default::default()
                            },
                            children![(
                                Text::new("Restart"),
                                TextColor(TEXT_COLOR),
                                TextFont::default().with_font_size(30.),
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                        .observe(on_hover_play_sound)
                        .observe(on_click_restart_button);

                    top_container
                        .spawn((
                            Button,
                            Node {
                                justify_content: JustifyContent::Center,
                                margin: UiRect::vertical(Val::Px(5.)),
                                width: Val::Percent(100.),
                                ..Default::default()
                            },
                            children![(
                                Text::new("Main Menu"),
                                TextColor(TEXT_COLOR),
                                TextFont::default().with_font_size(30.),
                                Pickable::IGNORE,
                            )],
                        ))
                        .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                        .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                        .observe(on_hover_play_sound)
                        .observe(on_click_go_to_main_menu_button);
                });
        });
}

fn on_click_resume_button(_trigger: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(LevelResume);
}
