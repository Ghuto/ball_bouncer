use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::{
    GameLayer,
    ball::{Ball, ModifyBall},
    controllable_plane::ModifyPlane,
    game_state::MainState,
};

const POWER_UP_PICK_UP_SPEED: f32 = -200.;
const POWER_UP_PICK_UP_RADIUS: f32 = 5.;

#[derive(Default)]
pub struct PowerUpPickUpProps {
    pub position: Vec3,
}

#[derive(SceneComponent, Default, Clone)]
#[scene(PowerUpPickUpProps)]
#[require(
    CollisionLayers::new(GameLayer::PickUp, GameLayer::ControllablePlane),
    RigidBody::Kinematic,
    LinearVelocity(Vec2::new(0., POWER_UP_PICK_UP_SPEED)),
    CollisionEventsEnabled,
    Collider::circle(POWER_UP_PICK_UP_RADIUS),
    Sensor
)]
pub struct PowerUpPickUp(PowerUp);

impl PowerUpPickUp {
    fn scene(props: PowerUpPickUpProps) -> impl Scene {
        let power_up = PowerUp::default();
        let color = power_up.color();

        bsn! {
            #PowerUp
            PowerUpPickUp(power_up)
            Transform::from_translation(props.position)
            Mesh2d(asset_value(Sphere::new(POWER_UP_PICK_UP_RADIUS)))
            MeshMaterial2d::<ColorMaterial>(asset_value(color))
            template(|ctx|{
                let game_state = ctx.resource::<State<MainState>>();
                Ok(DespawnOnExit::<MainState>(game_state.get().clone()))
            })
            on(PowerUpPickUp::on_pick_up)
        }
    }

    fn on_pick_up(
        collision: On<CollisionStart>,
        mut commands: Commands,
        power_up_pick_up: Query<&PowerUpPickUp>,
    ) {
        let pick_up_entity = collision.collider1;

        match &power_up_pick_up.get(pick_up_entity).unwrap().0 {
            PowerUp::SpawnBall => {
                commands.spawn_scene(bsn! {
                    @Ball{
                        @position: Vec3::new(0., 50., 0.)
                    }
                });
            }
            PowerUp::ModifyPlane(modification) => {
                commands.trigger(modification.clone());
            }
            PowerUp::ModifyBalls(modification) => {
                commands.trigger(modification.clone());
            }
        }
        commands.entity(pick_up_entity).despawn();
    }
}

#[derive(Clone)]
enum PowerUp {
    SpawnBall,
    ModifyBalls(ModifyBall),
    ModifyPlane(ModifyPlane),
}
impl Default for PowerUp {
    fn default() -> Self {
        let mut rng = rand::rng();
        rng.random::<PowerUp>()
    }
}

impl Distribution<PowerUp> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUp {
        let index: u8 = rng.random_range(0..5);
        match index {
            0 => PowerUp::SpawnBall,
            1 => PowerUp::ModifyBalls(ModifyBall::Accelerate),
            2 => PowerUp::ModifyBalls(ModifyBall::Decelerate),
            3 => PowerUp::ModifyPlane(ModifyPlane::Shorten),
            4 => PowerUp::ModifyPlane(ModifyPlane::Extend),
            _ => unreachable!(),
        }
    }
}

impl PowerUp {
    pub fn color(&self) -> Color {
        match self {
            PowerUp::SpawnBall => Color::Srgba(tailwind::YELLOW_500),
            PowerUp::ModifyBalls(ModifyBall::Accelerate) => Color::Srgba(tailwind::GREEN_500),
            PowerUp::ModifyBalls(ModifyBall::Decelerate) => Color::Srgba(tailwind::SKY_500),
            PowerUp::ModifyPlane(ModifyPlane::Shorten) => Color::Srgba(tailwind::RED_500),
            PowerUp::ModifyPlane(ModifyPlane::Extend) => Color::Srgba(tailwind::STONE_500),
        }
    }
}
