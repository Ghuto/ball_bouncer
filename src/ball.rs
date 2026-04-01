use avian2d::prelude::*;
use bevy::prelude::*;

use crate::playable_plane::*;
use crate::states::*;

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 100.;

#[derive(Component, Clone)]
#[require(
    RigidBody::Dynamic,
    Collider::circle(BALL_RADIUS),
    GravityScale(0.),
    TransformInterpolation,
    Friction{
        dynamic_coefficient : 0.,
        static_coefficient : 0.,
        combine_rule: CoefficientCombine::Min,
    },
    Restitution{
        coefficient : 1.,
        combine_rule: CoefficientCombine::Max,
    },
    DespawnOnExit::<GameState>(GameState::Playing)
)]
pub struct Ball;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    plane_transform: Single<&mut Transform, With<PlayablePlane>>,
) {
    let mut transform = plane_transform.clone();
    transform.translation.y = transform.translation.y + 50.;

    commands.spawn((
        Ball,
        LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED)),
        transform,
        Mesh2d(meshes.add(Sphere::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
    ));
}
