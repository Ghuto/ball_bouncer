use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameOver, MainState};

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
    DespawnOnExit::<MainState>(MainState::GamePlay),        
    LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED)),
)]
pub struct Ball;

#[derive(Event,Clone)]
pub struct SpawnBall{
    pub at_position: Vec3,
}

pub fn spawn_ball(
    trigger: On<SpawnBall>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

) {
    commands.spawn((
        Ball,
        Transform::from_translation(trigger.at_position),
        Mesh2d(meshes.add(Sphere::new(BALL_RADIUS))),
        MeshMaterial2d(materials.add(BALL_COLOR)),
    ));
}

pub fn watch_game_over_condition(mut commands: Commands,ball_q: Query<&Transform, With<Ball>>){

    for transform in ball_q{
        if transform.translation.y < 0.{
            commands.trigger(GameOver);
        }
    }

}
