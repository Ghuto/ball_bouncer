use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::MainState;

const BRICK_WIDTH: f32 = 30.;
const BRICK_HEIGHT: f32 = 20.;
const BRICK_COLOR: Color = bevy::prelude::Color::Srgba(tailwind::TEAL_400);

#[derive(Component)]
#[require(
    RigidBody::Static,
    DespawnOnExit::<MainState>(MainState::GamePlay),
    Collider::rectangle(BRICK_WIDTH, BRICK_HEIGHT),
    CollisionEventsEnabled,
)]
pub struct Brick;

#[derive(Event, Clone)]
pub struct SpawnBrick {
    pub at_position: Vec3,
}

pub fn on_spawn_brick(
    trigger: On<SpawnBrick>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Brick,
            Transform::from_translation(trigger.at_position),
            Mesh2d(meshes.add(Rectangle::new(BRICK_WIDTH, BRICK_HEIGHT))),
            MeshMaterial2d(materials.add(BRICK_COLOR)),
        ))
        .observe(on_collision_end);
}

fn on_collision_end(trigger: On<CollisionEnd>, mut commands: Commands) {
    commands.entity(trigger.collider1).despawn();
}
