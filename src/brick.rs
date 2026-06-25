use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::{MainState, general_events::CheckWinCondition};

const BRICK_WIDTH: f32 = 30.;
const BRICK_HEIGHT: f32 = 20.;

const BRICK_PADDING: f32 = 2.;
const BRICK_WIDTH_WITH_PADDING: f32 = BRICK_WIDTH + BRICK_PADDING;
const BRICK_HEIGHT_WITH_PADDING: f32 = BRICK_HEIGHT + BRICK_PADDING;

const BRICK_COLOR: Color = bevy::prelude::Color::Srgba(tailwind::TEAL_400);

#[derive(Component, Clone, Default, Reflect)]
#[reflect(Component)]
#[require(
    DespawnOnExit::<MainState>(MainState::Game),
    RigidBody::Static,
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
    brick_q: Query<&Transform, With<Brick>>,
) {
    let mut translation = trigger.at_position;

    fn round_position(position: f32, rounding_value: f32) -> f32 {
        return position - (position % rounding_value) + (rounding_value / 2. * position.signum());
    }

    // round the position so it would be spawn in grid
    translation.x = round_position(translation.x, BRICK_WIDTH_WITH_PADDING);
    translation.y = round_position(translation.y, BRICK_HEIGHT_WITH_PADDING);
    translation.z = 0.;

    // if there is a brick already in this position then do not spawn a brick
    for transform_brick in brick_q {
        if transform_brick.translation.eq(&translation) {
            return;
        }
    }

    commands.spawn((Brick, Transform::from_translation(translation)));
}

#[derive(Resource)]
pub struct BrickMesh {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

impl FromWorld for BrickMesh {
    fn from_world(world: &mut World) -> Self {
        BrickMesh {
            mesh_handle: world.add_asset::<Mesh>(Rectangle::new(BRICK_WIDTH, BRICK_HEIGHT)),
            material_handle: world.add_asset::<ColorMaterial>(BRICK_COLOR),
        }
    }
}

pub fn on_brick_insert(
    on_brick_insert: On<Insert, Brick>,
    mut commands: Commands,
    brick_mesh: Res<BrickMesh>,
) {
    commands
        .entity(on_brick_insert.entity)
        .insert((
            Mesh2d(brick_mesh.mesh_handle.clone()),
            MeshMaterial2d(brick_mesh.material_handle.clone()),
        ))
        // destroying brick
        .observe(|trigger: On<CollisionEnd>, mut commands: Commands| {
            commands.entity(trigger.collider1).despawn();
            commands.trigger(CheckWinCondition);
        });
}
