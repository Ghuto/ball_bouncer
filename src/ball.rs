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

#[derive(Component, Clone)]
pub struct Ball;

#[derive(Event, Clone)]
pub struct SpawnBall {
    pub at_position: Vec3,
}

pub fn spawn_ball(
    trigger: On<SpawnBall>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Ball,
            Transform::from_translation(trigger.at_position),
            Mesh2d(meshes.add(Sphere::new(BALL_RADIUS))),
            MeshMaterial2d(materials.add(BALL_COLOR)),
            RigidBody::Dynamic,
            Collider::circle(BALL_RADIUS),
            GravityScale(0.),
            TransformInterpolation,
            Friction {
                dynamic_coefficient: 0.,
                static_coefficient: 0.,
                combine_rule: CoefficientCombine::Min,
            },
            Restitution {
                coefficient: 1.,
                combine_rule: CoefficientCombine::Max,
            },
            DespawnOnExit(MainState::Game),
            LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED)),
            CollisionEventsEnabled,
        ))
        .observe(play_bounce_sound);
}

pub fn play_bounce_sound(_event: On<CollisionStart>, mut commands: Commands) {
    commands.trigger(PlaySoundEffect(SoundEffect::BallBounce));
}

pub fn despawn_lost_balls(mut commands: Commands, ball_q: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in ball_q {
        if transform.translation.y < BALL_DESPAWN_Y {
            commands.entity(entity).despawn();
        }
    }
}
