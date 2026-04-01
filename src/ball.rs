use avian2d::prelude::*;
use bevy::prelude::*;

use crate::playable_plane::*;
use crate::states::*;

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 100.;

#[derive(Component, Clone)]
pub struct Ball;

#[derive(Event, Clone)]
pub struct SpawnBall {
    pub transform: Transform,
    pub velocity: LinearVelocity,
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
        trigger.velocity,
        trigger.transform,
        Collider::circle(BALL_RADIUS),
        Mesh2d(meshes.add(Sphere::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::PERFECTLY_ELASTIC.with_combine_rule(CoefficientCombine::Max),
        GravityScale(0.),
        TransformInterpolation,
    ));
}

pub fn spawn_ball(
    mut commands: Commands,
    plane_transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    let mut transform = plane_transform.clone();
    transform.translation.y = transform.translation.y + 50.;

    commands.trigger(SpawnBall {
        velocity: LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED)),
        transform: transform,
    });
}
