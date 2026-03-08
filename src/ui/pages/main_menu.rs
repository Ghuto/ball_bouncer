use bevy::{color::palettes::tailwind, prelude::*};

use crate::ui::{
    on_click_play_button, on_click_quit_button, states::MenuPage, update_text_color_on_hover,
    update_text_color_on_hover_end,
};

pub const BORDER_COLOR: Color = Color::Srgba(tailwind::GRAY_700);
pub const TEXT_COLOR: Color = Color::Srgba(tailwind::SLATE_400);
pub const TEXT_HOVER_COLOR: Color = Color::Srgba(tailwind::SLATE_100);
pub const BACKGROUND_COLOR: Color = Color::Srgba(tailwind::ZINC_800);

pub fn build(mut commands: Commands, menu: Res<State<MenuPage>>) {
    commands
        .spawn((
            DespawnOnExit(menu.clone()),
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
                        .observe(update_text_color_on_hover(TEXT_HOVER_COLOR))
                        .observe(update_text_color_on_hover_end(TEXT_COLOR));

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
                        .observe(update_text_color_on_hover(TEXT_HOVER_COLOR))
                        .observe(update_text_color_on_hover_end(TEXT_COLOR));

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
                        .observe(update_text_color_on_hover(TEXT_HOVER_COLOR))
                        .observe(update_text_color_on_hover_end(TEXT_COLOR));
                });
        });
}
