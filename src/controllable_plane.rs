use avian2d::prelude::*;
use bevy::prelude::*;

use crate::MainState;

pub const PLANE_SPEED: f32 = 300.;
pub const PLANE_WIDTH: f32 = 100.;
pub const PLANE_HEIGHT: f32 = 10.;
pub const PLANE_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::BLACK);

pub const INPUT_LEFT: [KeyCode; 2] = [KeyCode::ArrowRight, KeyCode::KeyD];
pub const INPUT_RIGHT: [KeyCode; 2] = [KeyCode::ArrowLeft, KeyCode::KeyA];

#[derive(Component, Clone, Default)]
#[require(
    TransformInterpolation,
    DespawnOnExit::<MainState>(MainState::Game),
    Collider::rectangle(PLANE_WIDTH, PLANE_HEIGHT),
    LinearVelocity::ZERO,
    SweptCcd::default(),
    RigidBody::Kinematic,
)]
pub struct ControllablePlane;

#[derive(Event, Clone)]
pub struct SpawnControllablePlane {
    pub at_position: Vec3,
}

pub fn spawn_controllable_plane(
    trigger: On<SpawnControllablePlane>,
    mut commands: Commands,
    controllable_plane_q: Query<&ControllablePlane>,
) {
    // There can only be ONE!
    if !controllable_plane_q.is_empty() {
        return;
    }

    commands.spawn((
        ControllablePlane,
        Transform::from_translation(trigger.at_position),
    ));
}

pub fn control_plane(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    controllable_plane: Single<
        (&mut Transform, &Collider, &mut LinearVelocity, Entity),
        With<ControllablePlane>,
    >,
    move_and_slide: MoveAndSlide,
) {
    let (mut transform, collider, mut linear_velocity, entity) = controllable_plane.into_inner();

    // Adjust linear velocity based on input
    // for Avian's move and slide
    linear_velocity.0 = Vec2::ZERO;

    if input.any_pressed(INPUT_LEFT) {
        linear_velocity.0 += Vec2::X * PLANE_SPEED;
    }

    if input.any_pressed(INPUT_RIGHT) {
        linear_velocity.0 -= Vec2::X * PLANE_SPEED;
    }

    // Using Avian's move and slide. To handle kinematic
    // body collisions in case it hits a wall
    let MoveAndSlideOutput {
        position: new_position,
        projected_velocity: new_velocity,
    } = move_and_slide.move_and_slide(
        collider,
        transform.translation.xy(),
        Rotation::from(transform.rotation).as_radians(),
        linear_velocity.0,
        time.delta(),
        &MoveAndSlideConfig::default(),
        &SpatialQueryFilter::from_excluded_entities([entity]),
        |_| MoveAndSlideHitResponse::Accept,
    );

    // apply move and slide output
    linear_velocity.0 = new_velocity;
    transform.translation = new_position.extend(transform.translation.z);
}

#[derive(Resource)]
pub struct ControllablePlaneMesh {
    mesh_handle: Handle<Mesh>,
    material_handle: Handle<ColorMaterial>,
}

impl FromWorld for ControllablePlaneMesh {
    fn from_world(world: &mut World) -> Self {
        ControllablePlaneMesh {
            mesh_handle: world.add_asset::<Mesh>(Rectangle::new(PLANE_WIDTH, PLANE_HEIGHT)),
            material_handle: world.add_asset::<ColorMaterial>(PLANE_COLOR),
        }
    }
}

pub fn on_controllable_plane_insert(
    on_brick_insert: On<Insert, ControllablePlane>,
    mut commands: Commands,
    brick_mesh: Res<ControllablePlaneMesh>,
) {
    commands.entity(on_brick_insert.entity).insert((
        Mesh2d(brick_mesh.mesh_handle.clone()),
        MeshMaterial2d(brick_mesh.material_handle.clone()),
    ));
}
