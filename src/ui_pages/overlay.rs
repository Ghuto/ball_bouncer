use bevy::prelude::*;

use crate::pause::PauseGame;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::OverLay),
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::End,
                ..Default::default()
            },
            Pickable::IGNORE,
        ))
        .with_children(|container| {
            container
                .spawn(Button)
                .with_children(|button_container| {
                    button_container.spawn((
                        Text::new("||"),
                        TextColor(TEXT_COLOR),
                        TextFont::from_font_size(36.),
                    ));
                })
                .observe(change_color_on_over(TEXT_HOVER_COLOR,None))
                .observe(change_color_on_out(TEXT_COLOR,None))
                .observe(on_click_pause_button);
        });
}

fn on_click_pause_button(_trigger: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(PauseGame);
}
