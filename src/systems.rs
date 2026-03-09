use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::events::*;
use crate::states::*;

pub const PLANE_SPEED: f32 = 200.;
pub const PLANE_WIDTH: f32 = 100.;
pub const PLANE_HEIGHT: f32 = 10.;
pub const PLANE_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::BLACK);

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 500.;

pub struct Border {
    right: f32,
    left: f32,
    top: f32,
    bottom: f32,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((MyCamera, Transform::from_xyz(5000., 5000., 5000.)));
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
        trigger.ball.clone(),
        trigger.transform,
        Mesh2d(meshes.add(Sphere::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
    ));
}

pub fn spawn_ball(
    mut commands: Commands,
    plane_transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    let mut transform = plane_transform.clone();
    transform.translation.y = transform.translation.y + 50.;

    commands.trigger(SpawnBall {
        ball: Ball {
            velocity: Vec2::new(0.5, 0.5),
        },
        transform: transform,
    });
}

fn get_border(center_transform: &Transform, window: &Window) -> Border {
    let half_window_size = window.width() / 2.;
    let half_window_height = window.height() / 2.;

    Border {
        right: center_transform.translation.x + half_window_size,
        left: center_transform.translation.x - half_window_size,
        top: center_transform.translation.y + half_window_height,
        bottom: center_transform.translation.y - half_window_height,
    }
}

pub fn move_ball(
    mut ball_q: Query<(&mut Ball, &mut Transform)>,
    time: Res<Time>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera_transform: Single<&Transform, (With<MyCamera>, Without<Ball>)>,
) {
    let border = get_border(camera_transform.into_inner(), window.into_inner());

    for (mut ball, mut ball_transform) in &mut ball_q {
        ball_transform.translation.x += BALL_SPEED * ball.velocity.x * time.delta_secs();
        ball_transform.translation.y += BALL_SPEED * ball.velocity.y * time.delta_secs();

        let distance_outside_right_border =
            ball_transform.translation.x + BALL_RADIUS - border.right;
        let distance_outside_left_border = ball_transform.translation.x - BALL_RADIUS - border.left;
        let distance_outside_top_border = ball_transform.translation.y + BALL_RADIUS - border.top;
        let distance_outside_bottom_border =
            ball_transform.translation.y - BALL_RADIUS - border.bottom;

        if distance_outside_right_border > 0. {
            ball.velocity.x = -ball.velocity.x.abs();
            ball_transform.translation.x =
                border.right - distance_outside_right_border - BALL_RADIUS;
        } else if distance_outside_left_border < 0. {
            ball.velocity.x = ball.velocity.x.abs();
            ball_transform.translation.x = border.left - distance_outside_left_border + BALL_RADIUS;
        }

        if distance_outside_top_border > 0. {
            ball.velocity.y = -ball.velocity.y.abs();
            ball_transform.translation.y = border.top - distance_outside_top_border - BALL_RADIUS;
        } else if distance_outside_bottom_border < 0. {
            ball.velocity.y = ball.velocity.y.abs();
            ball_transform.translation.y =
                border.bottom - distance_outside_bottom_border + BALL_RADIUS;
        }
    }
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
        PlayablePlane,
        Mesh2d(meshes.add(Rectangle::new(PLANE_WIDTH, PLANE_HEIGHT))),
        MeshMaterial2d(materials.add(PLANE_COLOR)),
        camera_transform.clone(),
    ));
}

pub fn control_playable_plane(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut plane_transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    if input.any_pressed([KeyCode::ArrowRight, KeyCode::KeyD]) {
        plane_transform.translation.x += PLANE_SPEED * time.delta_secs();
    } else if input.any_pressed([KeyCode::ArrowLeft, KeyCode::KeyA]) {
        plane_transform.translation.x -= PLANE_SPEED * time.delta_secs();
    }
}

pub fn contain_plane_in_screen(
    window: Single<&Window, With<PrimaryWindow>>,
    camera_transform: Single<&Transform, (With<MyCamera>, Without<PlayablePlane>)>,
    mut plane_transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    let half_plane_width = PLANE_WIDTH / 2.;
    let half_plane_height = PLANE_HEIGHT / 2.;

    let border = get_border(camera_transform.into_inner(), window.into_inner());

    if plane_transform.translation.x + half_plane_width > border.right {
        plane_transform.translation.x = border.right - half_plane_width;
    } else if plane_transform.translation.x - half_plane_width < border.left {
        plane_transform.translation.x = border.left + half_plane_width;
    }

    if plane_transform.translation.y + half_plane_height > border.top {
        plane_transform.translation.y = border.top - half_plane_height;
    } else if plane_transform.translation.y - half_plane_height < border.bottom {
        plane_transform.translation.y = border.bottom + half_plane_height;
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
