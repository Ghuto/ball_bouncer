use bevy::prelude::*;

use crate::LevelPause;

use super::*;

pub fn build(mut commands: Commands) {
    commands
        .spawn((
            DespawnOnExit(MenuPage::Overlay),
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
                .spawn((
                    Button,
                    Node {
                        // override default AlignItems::Stretch
                        align_self: AlignSelf::Start,
                        ..Default::default()
                    },
                    children![(
                        Text::new("||"),
                        TextColor(TEXT_COLOR),
                        TextFont::from_font_size(36.),
                        Pickable::IGNORE,
                    )],
                ))
                .observe(on_event_update_ui_entity::<Over>(TEXT_HOVER_COLOR))
                .observe(on_event_update_ui_entity::<Out>(TEXT_COLOR))
                .observe(on_hover_play_sound)
                .observe(on_click_pause_button);
        });
}

fn on_click_pause_button(_trigger: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(LevelPause);
}
