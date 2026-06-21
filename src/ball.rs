use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    MainState,
    sounds::{PlaySoundEffect, SoundEffect},
};

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 100.;
pub const BALL_DESPAWN_Y: f32 = -500.;

#[derive(Component, Clone, Default)]
#[require(
    RigidBody::Dynamic,
    TransformInterpolation,
    GravityScale(0.),
    Friction {
        dynamic_coefficient: 0.,
        static_coefficient: 0.,
        combine_rule: CoefficientCombine::Min,
    },
    Restitution {
        coefficient: 1.,
        combine_rule: CoefficientCombine::Max,
    },
    CollisionEventsEnabled,
    DespawnOnExit::<MainState>(MainState::Game)
)]
pub struct Ball;

#[derive(Event, Clone)]
pub struct SpawnBall {
    pub at_position: Vec3,
}

pub fn spawn_ball(trigger: On<SpawnBall>, mut commands: Commands) {
    let position = trigger.at_position;

    commands.spawn_scene(bsn! {
        Ball
        Transform {translation : position}
        Mesh2d(asset_value(Sphere::new(BALL_RADIUS)))
        MeshMaterial2d::<ColorMaterial>(asset_value(BALL_COLOR))
        Collider::circle(BALL_RADIUS)
        LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED))
        on(|_event: On<CollisionStart>, mut commands: Commands|{
            commands.trigger(PlaySoundEffect(SoundEffect::BallBounce))
        })
    });
}

pub fn despawn_lost_balls(mut commands: Commands, ball_q: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in ball_q {
        if transform.translation.y < BALL_DESPAWN_Y {
            commands.entity(entity).despawn();
        }
    }
}
