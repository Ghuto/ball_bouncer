use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::components::*;
use crate::events::*;
use crate::states::*;

pub const PLANE_SPEED: f32 = 200.;
pub const PLANE_WIDTH: f32 = 100.;
pub const PLANE_HEIGHT: f32 = 10.;
pub const PLANE_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::BLACK);

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 100.;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((MyCamera, Transform::default()));
}

pub fn on_spawn_ball(
    trigger: On<SpawnBall>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<State<GameState>>,
) {
    commands.spawn((
        DespawnOnExit(game_state.clone()),
        Ball,
        RigidBody::Dynamic,
        GravityScale(0.),
        Ccd::enabled(),
        trigger.velocity,
        trigger.transform,
        Collider::ball(BALL_RADIUS),
        Mesh2d(meshes.add(Sphere::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Restitution {
            coefficient: 1.,
            combine_rule: CoefficientCombineRule::Max,
        },
        Friction {
            coefficient: 0.,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
}

pub fn spawn_ball(
    mut commands: Commands,
    plane_transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    let mut transform = plane_transform.clone();
    transform.translation.y = transform.translation.y + 50.;

    commands.trigger(SpawnBall {
        velocity: Velocity {
            linvel: Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED),
            angvel: 0.,
        },
        transform: transform,
    });
}

pub fn spawn_playable_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    game_state: Res<State<GameState>>,
    camera_transform: Single<&Transform, (With<MyCamera>, Without<Ball>)>,
) {
    commands.spawn((
        DespawnOnExit(game_state.clone()),
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PLANE_WIDTH / 2., PLANE_HEIGHT / 2.),
        PlayablePlane,
        KinematicCharacterController::default(),
        Mesh2d(meshes.add(Rectangle::new(PLANE_WIDTH, PLANE_HEIGHT))),
        MeshMaterial2d(materials.add(PLANE_COLOR)),
        camera_transform.clone(),
        LockedAxes::ROTATION_LOCKED,
    ));
}

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
        RigidBody::Fixed,
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

pub fn control_playable_plane(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut plane: Single<&mut KinematicCharacterController, With<PlayablePlane>>,
) {
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        plane.translation = Some(vec2(PLANE_SPEED * time.delta_secs(), 0.));
    } else if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        plane.translation = Some(vec2(-PLANE_SPEED * time.delta_secs(), 0.));
    }
}

pub fn watching_input_to_pause_or_resume(mut commands: Commands) {
    commands.trigger(TogglePausedState);
}

pub fn on_toggle_pause_state(
    _: On<TogglePausedState>,
    mut pause_state_writer: ResMut<NextState<PausedState>>,
    pause_state: Res<State<PausedState>>,
) {
    match pause_state.get() {
        PausedState::Playing => pause_state_writer.set(PausedState::Paused),
        PausedState::Paused => pause_state_writer.set(PausedState::Playing),
    }
}
