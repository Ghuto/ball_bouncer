use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameLayer, MainState,
    general_events::CheckGameOverCondition,
    sounds::{PlaySoundEffect, SoundEffect},
};

pub const BALL_RADIUS: f32 = 5.;
pub const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
pub const BALL_SPEED: f32 = 100.;
pub const BALL_DESPAWN_Y: f32 = -500.;

pub const BALL_SPEED_ACCELERATE: f32 = BALL_SPEED / 4.;
pub const BALL_SPEED_DECELERATE: f32 = BALL_SPEED / 5.;
pub const BALL_SPEED_MIN: f32 = 30.;

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
    CollisionLayers::new(
        GameLayer::Ball,
        [GameLayer::ControllablePlane,GameLayer::Brick,GameLayer::Border],
    )
)]
pub struct Ball;

#[derive(Event, Clone)]
pub struct SpawnBall {
    pub at_position: Vec3,
    pub disabled: bool,
}

pub fn spawn_ball(trigger: On<SpawnBall>, mut commands: Commands, brick_mesh: Res<BallMesh>) {
    commands
        .spawn((
            Ball,
            Transform::from_translation(trigger.at_position),
            Mesh2d(brick_mesh.mesh_handle.clone()),
            MeshMaterial2d(brick_mesh.material_handle.clone()),
        ))
        .insert_if(RigidBodyDisabled, || trigger.disabled)
        .observe(|_event: On<CollisionStart>, mut commands: Commands| {
            commands.trigger(PlaySoundEffect(SoundEffect::BallBounce))
        });
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

#[derive(Copy, Clone)]
pub enum BallModification {
    Accelerate,
    Decelerate,
}

#[derive(Event)]
pub struct ModifyBall(pub BallModification);

pub fn modify_ball(
    modify_plane: On<ModifyBall>,
    mut ball_q: Query<&mut LinearVelocity, With<Ball>>,
) {
    let amount = match modify_plane.0 {
        BallModification::Accelerate => BALL_SPEED_ACCELERATE,
        BallModification::Decelerate => BALL_SPEED_DECELERATE,
    };

    for mut linear_velocity in ball_q.iter_mut() {
        linear_velocity.0 *= amount;
        let length = linear_velocity.length();
        if length < BALL_SPEED_MIN {
            let factor = linear_velocity.0 / length;
            linear_velocity.0 = factor * BALL_SPEED_MIN;
        }
    }
}
