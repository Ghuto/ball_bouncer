use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::MainState;

#[derive(Event, Clone)]
pub struct SpawnBorder;

pub fn spawn_border(
    _: On<SpawnBorder>,
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let half_window_size = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    let right = half_window_size;
    let left = -half_window_size;
    let top = half_window_height;
    let bottom = -half_window_height;

    commands.spawn((
        DespawnOnExit(MainState::GamePlay),
        RigidBody::Static,
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
