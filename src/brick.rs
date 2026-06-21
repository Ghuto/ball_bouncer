use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::MainState;

const BRICK_WIDTH: f32 = 30.;
const BRICK_HEIGHT: f32 = 20.;
const BRICK_COLOR: Color = bevy::prelude::Color::Srgba(tailwind::TEAL_400);

#[derive(Component, Clone, Default)]
#[require(
    RigidBody::Static,
    DespawnOnExit::<MainState>(MainState::Game),
    Collider::rectangle(BRICK_WIDTH, BRICK_HEIGHT),
    CollisionEventsEnabled,
)]
pub struct Brick;

#[derive(Event, Clone)]
pub struct SpawnBrick {
    pub at_position: Vec3,
}

pub fn on_spawn_brick(trigger: On<SpawnBrick>, mut commands: Commands) {
    let position = trigger.at_position;

    commands.spawn_scene(bsn! {
        Brick
        Transform {translation: position}
        Mesh2d(asset_value(Rectangle::new(BRICK_WIDTH, BRICK_HEIGHT)))
        MeshMaterial2d::<ColorMaterial>(asset_value(BRICK_COLOR))
        // destroying brick
        on(|trigger: On<CollisionEnd>, mut commands: Commands|{
            commands.entity(trigger.collider1).despawn();
        })
    });
}
