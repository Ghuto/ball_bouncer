use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameLayer, MainState};

const PLANE_SPEED: f32 = 300.;
const PLANE_WIDTH: f32 = 100.;
const PLANE_HEIGHT: f32 = 10.;
const PLANE_COLOR: Color = Color::Srgba(bevy::color::palettes::basic::BLACK);

const PLANE_SCALE_EXTEND_AMOUNT: f32 = 0.5;
const PLANE_SCALE_SHORTEN_AMOUNT: f32 = -0.3;
const PLANE_SCALE_MIN: f32 = 0.2;

const INPUT_LEFT: [KeyCode; 2] = [KeyCode::ArrowRight, KeyCode::KeyD];
const INPUT_RIGHT: [KeyCode; 2] = [KeyCode::ArrowLeft, KeyCode::KeyA];

#[derive(SceneComponent, Clone, Default)]
#[scene(ControllablePlaneProps)]
#[require(
    TransformInterpolation,
    Collider::rectangle(PLANE_WIDTH, PLANE_HEIGHT),
    LinearVelocity::ZERO,
    SweptCcd::default(),
    RigidBody::Kinematic,
    CollisionLayers::new([GameLayer::ControllablePlane,GameLayer::Default], [GameLayer::Default,GameLayer::PickUp,]),
)]
pub struct ControllablePlane;

pub struct ControllablePlaneProps {
    pub position: Vec3,
}

impl Default for ControllablePlaneProps {
    fn default() -> Self {
        ControllablePlaneProps {
            position: Vec3::new(0., -250., 0.),
        }
    }
}

impl ControllablePlane {
    fn scene(props: ControllablePlaneProps) -> impl Scene {
        bsn! {
            #ControllablePlane
            ControllablePlane
            Transform::from_translation(props.position)
            template(|ctx|{
                let controllable_plane_mesh = ctx.resource::<ControllablePlaneMesh>();
                Ok(Mesh2d(controllable_plane_mesh.mesh_handle.clone()))
            })
            template(|ctx|{
                let controllable_plane_mesh = ctx.resource::<ControllablePlaneMesh>();
                Ok(MeshMaterial2d(controllable_plane_mesh.material_handle.clone()))
            })
            template(|ctx|{
                let game_state = ctx.resource::<State<MainState>>();
                Ok(DespawnOnExit::<MainState>(game_state.get().clone()))
            })
        }
    }
}

#[derive(Event, Clone)]
pub enum ModifyPlane {
    Extend,
    Shorten,
}

impl ModifyPlane {
    pub fn on_trigger(
        modify_plane: On<Self>,
        mut plane_q: Single<&mut Transform, With<ControllablePlane>>,
    ) {
        let amount = match modify_plane.event() {
            ModifyPlane::Extend => PLANE_SCALE_EXTEND_AMOUNT,
            ModifyPlane::Shorten => PLANE_SCALE_SHORTEN_AMOUNT,
        };
        let new_scale = plane_q.scale.x + amount;

        plane_q.scale.x = if new_scale > PLANE_SCALE_MIN {
            new_scale
        } else {
            PLANE_SCALE_MIN
        }
    }
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
