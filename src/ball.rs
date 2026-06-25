use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    MainState,
    general_events::CheckGameOverCondition,
    sounds::{PlaySoundEffect, SoundEffect},
};

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 100.;
pub const BALL_DESPAWN_Y: f32 = -500.;

#[derive(Component, Clone, Default)]
#[require(
    DespawnOnExit::<MainState>(MainState::Game),
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
    Collider::circle(BALL_RADIUS),
    LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED)),
)]
pub struct Ball;

#[derive(Event, Clone)]
pub struct SpawnBall {
    pub at_position: Vec3,
}

pub fn spawn_ball(trigger: On<SpawnBall>, mut commands: Commands) {
    commands.spawn((
        Ball,
        Transform::from_translation(trigger.at_position),
        RigidBodyDisabled,
    ));
}

pub fn despawn_lost_balls(mut commands: Commands, ball_q: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in ball_q {
        if transform.translation.y < BALL_DESPAWN_Y {
            commands.entity(entity).despawn();
            commands.trigger(CheckGameOverCondition);
        }
    }
}

#[derive(Resource)]
pub struct BallMesh {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

impl FromWorld for BallMesh {
    fn from_world(world: &mut World) -> Self {
        BallMesh {
            mesh_handle: world.add_asset::<Mesh>(Sphere::new(BALL_RADIUS)),
            material_handle: world.add_asset::<ColorMaterial>(BALL_COLOR),
        }
    }
}

pub fn on_ball_insert(
    on_brick_insert: On<Insert, Ball>,
    mut commands: Commands,
    brick_mesh: Res<BallMesh>,
) {
    commands
        .entity(on_brick_insert.entity)
        .insert((
            Mesh2d(brick_mesh.mesh_handle.clone()),
            MeshMaterial2d(brick_mesh.material_handle.clone()),
        ))
        .observe(|_event: On<CollisionStart>, mut commands: Commands| {
            commands.trigger(PlaySoundEffect(SoundEffect::BallBounce))
        });
}
