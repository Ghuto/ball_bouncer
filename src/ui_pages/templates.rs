use bevy::prelude::*;

use crate::level::Level;

use super::buttons::*;
use super::*;

pub fn level_status_page(
    title_text: &'static str,
    color: Color,
    buttons: impl SceneList,
) -> impl Scene {
    bsn! {
        template(|ctx|{
            let state = ctx.resource::<State<MenuPage>>();
            Ok(DespawnOnExit::<MenuPage>(state.get().clone()))
        })
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
        }
        BackgroundColor(background_color(0.6))
        Pickable::IGNORE
        Children[
            (
                Node {
                    height: Val::Percent(40.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                }
                Children[(
                        Text::new(title_text)
                        TextFont {font_size: px(60.)}
                        TextColor(color)
                    )]
            ),(
                Node {
                    height: Val::Percent(40.),
                    width: Val::Percent(100.),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                }
                Children[{buttons}]
        ),]
    }
}

pub fn overlay_page() -> impl Scene {
    bsn! {
        template(|ctx|{
            let state = ctx.resource::<State<MenuPage>>();
            Ok(DespawnOnExit::<MenuPage>(state.get().clone()))
        })
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::End,
        }
        Pickable::IGNORE
        Children[(
            pause_icon_button()
        )]
    }
}

pub fn title_page() -> impl Scene {
    bsn! {
        template(|ctx|{
            let state = ctx.resource::<State<MenuPage>>();
            Ok(DespawnOnExit::<MenuPage>(state.get().clone()))
        })
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Start,
        }
        BackgroundColor(background_color(1.))
        Children[(
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Start,
                height: Val::Percent(100.),
                width: Val::Px(250.),
                border: UiRect::right(Val::Px(5.)),
            }
            BorderColor::all(BORDER_COLOR)
            Children[(
                level_selection_button()
            ),(
                Node {
                    width: Val::Percent(100.),
                    padding: UiRect::left(Val::Px(10.)),
                }
                menu_button("Settings",30.)
            ),(
                quit_button()
            )]
        )]

    }
}

pub fn level_selection_page() -> impl Scene {
    bsn! {
        template(|ctx|{
            let state = ctx.resource::<State<MenuPage>>();
            Ok(DespawnOnExit::<MenuPage>(state.get().clone()))
        })
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Start,
        }
        BackgroundColor(background_color(1.))
        Children[
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Start,
                    height: Val::Percent(100.),
                    width: Val::Px(250.),
                    border: UiRect::right(Val::Px(5.)),

                }
                BorderColor::all(BORDER_COLOR)
                Children[
                    {Level::get_list_of_level_buttons()}
                    ,(
                        return_button()
                    )
                ]
            )
        ]

    }
}
