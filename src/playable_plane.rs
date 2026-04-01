use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{camera::MyCamera, states::GameState};

pub const PLANE_SPEED: f32 = 200.;
pub const PLANE_WIDTH: f32 = 100.;
pub const PLANE_HEIGHT: f32 = 10.;
pub const PLANE_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::BLACK);

pub const INPUT_LEFT: [KeyCode; 2] = [KeyCode::ArrowRight, KeyCode::KeyD];
pub const INPUT_RIGHT: [KeyCode; 2] = [KeyCode::ArrowLeft, KeyCode::KeyA];

#[derive(Component)]
pub struct PlayablePlane;

pub fn spawn_playable_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<State<GameState>>,
    camera_transform: Single<&Transform, With<MyCamera>>,
) {
    commands.spawn((
        PlayablePlane,
        TransformInterpolation,
        DespawnOnExit(game_state.clone()),
        camera_transform.clone(),
        Mesh2d(meshes.add(Rectangle::new(PLANE_WIDTH, PLANE_HEIGHT))),
        MeshMaterial2d(materials.add(PLANE_COLOR)),
        RigidBody::Kinematic,
        Collider::rectangle(PLANE_WIDTH, PLANE_HEIGHT),
        LinearVelocity::ZERO,
        SweptCcd::default(),
    ));
}

pub fn control_playable_plane(
    input: Res<ButtonInput<KeyCode>>,
    mut plane_linear_velocity: Single<&mut LinearVelocity, With<PlayablePlane>>,
) {
    let mut movement = Vec2::ZERO;

    if input.any_pressed(INPUT_LEFT) {
        movement += Vec2::X;
    }

    if input.any_pressed(INPUT_RIGHT) {
        movement -= Vec2::X;
    }

    movement *= PLANE_SPEED;

    plane_linear_velocity.0 = movement;
}
