use avian2d::prelude::*;
use bevy::{color::palettes::tailwind, prelude::*};

use crate::{MainState, powerup::PowerUpPickUp};

const BRICK_WIDTH: f32 = 30.;
const BRICK_HEIGHT: f32 = 20.;

#[cfg(feature = "editor")]
const BRICK_PADDING: f32 = 2.;
#[cfg(feature = "editor")]
const BRICK_WIDTH_WITH_PADDING: f32 = BRICK_WIDTH + BRICK_PADDING;
#[cfg(feature = "editor")]
const BRICK_HEIGHT_WITH_PADDING: f32 = BRICK_HEIGHT + BRICK_PADDING;

const BRICK_COLOR: Color = bevy::prelude::Color::Srgba(tailwind::TEAL_400);

#[cfg_attr(feature = "editor", derive(SceneComponent), scene(Vec3))]
#[cfg_attr(not(feature = "editor"), derive(Component))]
#[derive(Clone, Default, Reflect)]
#[reflect(Component)]
#[require(
    RigidBody::Static,
    Collider::rectangle(BRICK_WIDTH, BRICK_HEIGHT),
    CollisionEventsEnabled,
    Name::new("Brick")
)]
pub struct Brick;

impl Brick {
    pub fn on_insert(
        on_brick_insert: On<Insert, Brick>,
        mut commands: Commands,
        brick_mesh: Res<BrickMesh>,
        main_state: Res<State<MainState>>,
    ) {
        commands
            .entity(on_brick_insert.entity)
            .insert((
                Mesh2d(brick_mesh.mesh_handle.clone()),
                MeshMaterial2d(brick_mesh.material_handle.clone()),
                DespawnOnExit::<MainState>(main_state.get().clone()),
            ))
            .observe(Self::destroy);
    }

    /// on collision destroys the brick
    fn destroy(trigger: On<CollisionEnd>, mut commands: Commands, brick_q: Query<&Transform>) {
        commands.entity(trigger.collider1).despawn();

        // random chance to psawn a power up pickup
        if rand::random::<bool>() {
            let translation = brick_q.get(trigger.collider1).unwrap().translation;

            commands.spawn_scene(bsn! {
                @PowerUpPickUp{
                    @position: translation,
                }
            });
        }
    }

    #[cfg(feature = "editor")]
    fn scene(position: Vec3) -> impl Scene {
        bsn! {
            Transform {translation: position}
        }
    }

    #[cfg(feature = "editor")]
    pub fn rounded_position(mut position: Vec3) -> Vec3 {
        fn round(position: f32, rounding_value: f32) -> f32 {
            return position - (position % rounding_value)
                + (rounding_value / 2. * position.signum());
        }

        // round the position so it would be spawn in grid
        position.x = round(position.x, BRICK_WIDTH_WITH_PADDING);
        position.y = round(position.y, BRICK_HEIGHT_WITH_PADDING);

        position
    }
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
