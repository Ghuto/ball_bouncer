use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{MainState, sounds::SoundEffect};

const BALL_RADIUS: f32 = 5.;
const BALL_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::WHITE);
const BALL_SPEED: f32 = 100.;
const BALL_DESPAWN_Y: f32 = -500.;
const BALL_SPEED_ACCELERATE: f32 = BALL_SPEED / 3.;
const BALL_SPEED_DECELERATE: f32 = BALL_SPEED / 4.;
const BALL_SPEED_MIN: f32 = 30.;

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

#[derive(SceneComponent, Clone, Default)]
#[scene(BallProps)]
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
    Collider::circle(BALL_RADIUS),
    LinearVelocity(Vec2::new(3. * BALL_SPEED, 2. * BALL_SPEED)),
)]
pub struct Ball;

pub struct BallProps {
    pub position: Vec3,
}

impl Default for BallProps {
    fn default() -> Self {
        BallProps {
            position: Vec3::new(0., 50., 0.),
        }
    }
}

impl Ball {
    fn scene(props: BallProps) -> impl Scene {
        let position = props.position;
        bsn! {
            #Ball
            Transform {translation: position}
            template(|ctx|{
                let game_state = ctx.resource::<State<MainState>>();
                Ok(DespawnOnExit::<MainState>(game_state.get().clone()))
            })
            template(|ctx|{
                let ball_mesh = ctx.resource::<BallMesh>();
                Ok(Mesh2d(ball_mesh.mesh_handle.clone()))
            })
            template(|ctx|{
                let ball_mesh = ctx.resource::<BallMesh>();
                Ok(MeshMaterial2d(ball_mesh.material_handle.clone()))
            })
            // on bounce
            on(|_event: On<CollisionStart>, mut commands: Commands| {
                commands.trigger(SoundEffect::BallBounce)
            })
        }
    }
}

pub fn despawn_lost_balls(mut commands: Commands, ball_q: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in ball_q {
        if transform.translation.y < BALL_DESPAWN_Y {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Event, Clone)]
pub enum ModifyBall {
    Accelerate,
    Decelerate,
}

impl ModifyBall {
    pub fn on_trigger(modify_plane: On<Self>, mut ball_q: Query<&mut LinearVelocity, With<Ball>>) {
        let ball_min_speed = BALL_SPEED_MIN;
        let amount = modify_plane.amount();

        for mut linear_velocity in ball_q.iter_mut() {
            let length = linear_velocity.length();
            let factor = linear_velocity.0 / length;

            if length < ball_min_speed {
                linear_velocity.0 = factor * ball_min_speed;
            } else {
                linear_velocity.0 += amount * factor;
            }
        }
    }

    fn amount(&self) -> f32 {
        match &self {
            ModifyBall::Accelerate => BALL_SPEED_ACCELERATE,
            ModifyBall::Decelerate => BALL_SPEED_DECELERATE,
        }
    }
}
