use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};
use rand::{
    Rng, RngExt,
    distr::{Distribution, StandardUniform},
};

use crate::{
    GameLayer,
    ball::{BallModification, SpawnBall},
    controllable_plane::{ModifyPlane, PlaneModification},
    game_state::MainState,
};

const POWER_UP_PICK_UP_SPEED: f32 = -200.;
const POWER_UP_PICK_UP_RADIUS: f32 = 5.;

#[derive(Component)]
#[require(
    DespawnOnExit::<MainState>(MainState::Game),
    CollisionLayers::new(GameLayer::PickUp, [GameLayer::ControllablePlane,]),
    RigidBody::Kinematic,
    LinearVelocity(Vec2::new(0., POWER_UP_PICK_UP_SPEED)),
    CollisionEventsEnabled,
    Collider::circle(POWER_UP_PICK_UP_RADIUS),
)]
struct PowerUpPickUp(pub PowerUp);

enum PowerUp {
    SpawnBall,
    ModifyBalls(BallModification),
    ModifyPlane(PlaneModification),
}

impl Distribution<PowerUp> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PowerUp {
        let index: u8 = rng.random_range(0..5);
        match index {
            0 => PowerUp::SpawnBall,
            1 => PowerUp::ModifyBalls(BallModification::Accelerate),
            2 => PowerUp::ModifyBalls(BallModification::Decelerate),
            3 => PowerUp::ModifyPlane(PlaneModification::Shorten),
            4 => PowerUp::ModifyPlane(PlaneModification::Extend),
            _ => unreachable!(),
        }
    }
}

impl PowerUp {
    pub fn color(&self) -> Color {
        match self {
            PowerUp::SpawnBall => Color::Srgba(tailwind::YELLOW_500),
            PowerUp::ModifyBalls(BallModification::Accelerate) => Color::Srgba(tailwind::GREEN_500),
            PowerUp::ModifyBalls(BallModification::Decelerate) => Color::Srgba(tailwind::SKY_500),
            PowerUp::ModifyPlane(PlaneModification::Shorten) => Color::Srgba(tailwind::RED_500),
            PowerUp::ModifyPlane(PlaneModification::Extend) => Color::Srgba(tailwind::STONE_500),
        }
    }
}

#[derive(Event)]
pub struct TrySpawnPowerUpPickUp {
    pub at_position: Vec3,
}

pub fn try_to_spawn_power_up_pick_up(
    try_spawn_power_up: On<TrySpawnPowerUpPickUp>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::rng();
    let power_up = rng.random::<PowerUp>();
    if rng.random::<bool>() {
        commands
            .spawn((
                Transform::from_translation(try_spawn_power_up.at_position),
                Mesh2d(meshes.add(Sphere::new(POWER_UP_PICK_UP_RADIUS))),
                MeshMaterial2d(materials.add(power_up.color())),
                PowerUpPickUp(power_up),
            ))
            .observe(
                |collision: On<CollisionStart>,
                 mut commands: Commands,
                 power_up_pick_up: Query<&PowerUpPickUp>| {
                    let pick_up_entity = collision.collider1;

                    match power_up_pick_up.get(pick_up_entity).unwrap().0 {
                        PowerUp::SpawnBall => {
                            commands.trigger(SpawnBall {
                                at_position: Vec3::new(0., 50., 0.),
                                disabled: false,
                            });
                        }
                        PowerUp::ModifyPlane(modification) => {
                            commands.trigger(ModifyPlane(modification));
                        }
                        _ => {}
                    }
                    commands.entity(pick_up_entity).despawn();
                },
            );
    }
}
