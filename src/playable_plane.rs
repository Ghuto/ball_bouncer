use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{camera::MyCamera, states::GameState};

pub const PLANE_SPEED: f32 = 300.;
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
    time: Res<Time>,
    playable_plane: Single<
        (&mut Transform, &Collider, &mut LinearVelocity, Entity),
        With<PlayablePlane>,
    >,
    move_and_slide: MoveAndSlide,
) {
    let (mut transform, collider, mut linear_velocity, entity) = playable_plane.into_inner();

    // Adjust linear velocity based on input
    // for Avian's move and slide
    linear_velocity.0 = Vec2::ZERO;

    if input.any_pressed(INPUT_LEFT) {
        linear_velocity.0 += Vec2::X * PLANE_SPEED;
    }

    if input.any_pressed(INPUT_RIGHT) {
        linear_velocity.0 -= Vec2::X * PLANE_SPEED;
    }

    // Using Avian's move and slide. To handle kinematic
    // body collisions. In case it hits a wall
    let MoveAndSlideOutput {
        position: new_position,
        projected_velocity: new_velocity,
    } = move_and_slide.move_and_slide(
        collider,
        transform.translation.xy(),
        Rotation::from(transform.rotation).as_radians(),
        linear_velocity.0,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &SpatialQueryFilter::from_excluded_entities([entity]),
        |_| MoveAndSlideHitResponse::Accept,
    );

    // apply move and slide output
    linear_velocity.0 = new_velocity;
    transform.translation = new_position.extend(transform.translation.z);
}
