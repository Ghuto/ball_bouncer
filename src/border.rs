use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{camera::MyCamera, playable_plane::PlayablePlane, states::GameState};

pub fn spawn_border(
    mut commands: Commands,
    camera_transform: Single<&Transform, (With<MyCamera>, Without<PlayablePlane>)>,
    window: Single<&Window, With<PrimaryWindow>>,
    game_state: Res<State<GameState>>,
) {
    let half_window_size = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    let right = half_window_size;
    let left = -half_window_size;
    let top = half_window_height;
    let bottom = -half_window_height;

    commands.spawn((
        DespawnOnExit(game_state.clone()),
        RigidBody::Static,
        camera_transform.clone(),
        Collider::polyline(
            vec![
                vec2(left, bottom),
                vec2(left, top),
                vec2(right, top),
                vec2(right, bottom),
            ],
            None,
        ),
    ));
}
