use avian2d::prelude::*;
use bevy::prelude::*;

use crate::MainState;

#[derive(SceneComponent, Default, Clone)]
#[scene(LevelBorderProps)]
pub struct LevelBorder;

#[derive(Default)]
pub struct LevelBorderProps {
    pub border_vertices: Vec<Vec2>,
}

impl LevelBorder {
    pub fn scene(props: LevelBorderProps) -> impl Scene {
        bsn! {
            #LevelBorder
            template_value(RigidBody::Static)
            Collider::polyline(
                props.border_vertices,
                None,
            )
            template(|ctx|{
                let game_state = ctx.resource::<State<MainState>>();
                Ok(DespawnOnExit::<MainState>(game_state.get().clone()))
            })
        }
    }

    pub fn vertices_from_window(window: &Window) -> Vec<Vec2> {
        let half_window_width = window.width() / 2.;
        let half_window_height = window.height() / 2.;

        // window sides
        let right = half_window_width;
        let left = -half_window_width;
        let top = half_window_height;
        let bottom = -half_window_height;

        vec![
            vec2(left, bottom),
            vec2(left, top),
            vec2(right, top),
            vec2(right, bottom),
        ]
    }
}
