use std::ops::Deref;

#[cfg(not(target_arch = "wasm32"))]
use bevy::tasks::IoTaskPool;
use bevy::{
    camera::RenderTarget,
    feathers::{
        FeathersPlugins, containers::flex_spacer, controls::*, dark_theme::create_dark_theme,
        display::label_dim, theme::*,
    },
    platform::collections::HashSet,
    prelude::*,
    ui::{Checked, InteractionDisabled},
    ui_widgets::{Activate, ValueChange, checkbox_self_update},
    window::{PrimaryWindow, WindowRef, WindowResolution},
};

use std::fs::File;
use std::io::Write;

use crate::{brick::Brick, camera::MyCamera, game_state::MainState, level::Level};

pub struct EditorPlugin;

#[derive(Resource)]
struct EditorSettings {
    pub allow_spawning_items_on_click: bool,
}

#[derive(Component, Clone, Default)]
struct ConditionalWidget();

fn allow_spawning_items_on_click(settings: Res<EditorSettings>) -> bool {
    settings.allow_spawning_items_on_click
}

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FeathersPlugins)
            .insert_resource(UiTheme(create_dark_theme()))
            .insert_resource(EditorSettings {
                allow_spawning_items_on_click: false,
            })
            .init_state::<EditingState>()
            .add_systems(Startup, editor_window)
            .add_systems(OnEnter(EditingState::Saving), saving_level_into_scene)
            .add_systems(
                Update,
                (
                    on_mouse_click_spawn_brick.run_if(allow_spawning_items_on_click),
                    control_conditional_widgets.run_if(state_changed::<MainState>),
                ),
            );
    }
}

fn control_conditional_widgets(
    mut commands: Commands,
    widget_q: Query<Entity, With<ConditionalWidget>>,
    main_state: Res<State<MainState>>,
) {
    if main_state.eq(&MainState::Title) {
        for entity in widget_q {
            commands
                .entity(entity)
                .insert(InteractionDisabled)
                .remove::<Checked>();
        }
    } else {
        for entity in widget_q {
            commands.entity(entity).remove::<InteractionDisabled>();
        }
    }
}

pub fn on_mouse_click_spawn_brick(
    mut commands: Commands,
    window: Single<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    camera_transform: Single<&Transform, With<MyCamera>>,
    brick_q: Query<&Transform, With<Brick>>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(cursor_position) = window.cursor_position() {
            let camera_translation = camera_transform.translation;

            // Cursor location starts from left/top to right/bottom.
            // Camera origin is center but cursor origin is left/top
            // therefor subtracting half of the screen.
            // Y needs to be inverted to go from top->bottom to bottom->top.
            let mut brick_position_x =
                cursor_position.x - window.width() / 2. + camera_translation.x;
            let mut brick_position_y =
                window.height() / 2. - cursor_position.y + camera_translation.y;
            let mut brick_position_z = 0.;

            let rounded_translation = Brick::rounded_position(Vec3::new(
                brick_position_x,
                brick_position_y,
                brick_position_z,
            ));

            // if there is a brick already in this position then do not spawn a brick
            for transform_brick in brick_q {
                if transform_brick.translation.eq(&rounded_translation) {
                    return;
                }
            }

            brick_position_x = rounded_translation.x;
            brick_position_y = rounded_translation.y;
            brick_position_z = rounded_translation.z;

            commands.spawn_scene(bsn! {
                @Brick{
                    @x: brick_position_x ,
                    @y: brick_position_y,
                    @z: brick_position_z,
                }
            });
        }
    }
}

pub fn editor_window(mut commands: Commands) {
    let secodary_window_entity = commands
        .spawn(Window {
            title: "Second window".to_owned(),
            resolution: WindowResolution::new(300, 600),
            ..Default::default()
        })
        .id();

    let second_window_camera = commands
        .spawn((
            Camera2d::default(),
            Transform::from_xyz(6.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            RenderTarget::Window(WindowRef::Entity(secodary_window_entity)),
        ))
        .id();

    commands
        .spawn_scene(bsn! {
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Stretch,
                justify_content: JustifyContent::Start,
                row_gap: px(8),
                width: percent(100),
            }
            Children[
                (
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        column_gap: px(8),
                    }
                    Children [
                        (
                            @FeathersToggleSwitch
                            ConditionalWidget
                            on(checkbox_self_update)
                            on(|value_change: On<ValueChange<bool>>,mut editor_setting:ResMut<EditorSettings>| {
                                editor_setting.allow_spawning_items_on_click = value_change.value;
                            })
                        ),
                        (label_dim("Spawning bricks on click"))
                    ]
                )
                flex_spacer(),
                (
                    label_dim("Level scene:")
                ),
                (
                    Node {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        column_gap: px(8),
                    }
                    Children [
                        (
                            @FeathersButton {
                                @caption: bsn! { Text("Save") ThemedText }
                            }
                            ConditionalWidget
                            Node {
                                flex_grow: 1.0,
                            }
                            on(|_activate: On<Activate>, mut editing_state: ResMut<NextState<EditingState>>| {
                                editing_state.set(EditingState::Saving);
                            })
                        ),
                    ]
                ),
            ]
        })
        // UiTargetCamera does not derive default therefor adding seperatly
        .insert(UiTargetCamera(second_window_camera));
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
enum EditingState {
    #[default]
    Editing,
    Saving,
}

fn saving_level_into_scene(world: &mut World) {
    let mut query = QueryBuilder::<Entity, With<Brick>>::new(world).build();

    let entities: HashSet<Entity> = query.iter(world).collect();

    if let Some(mut editing_state) = world.get_resource_mut::<NextState<EditingState>>() {
        editing_state.set(EditingState::Editing);
    }

    let level_path = world
        .query::<&Level>()
        .single(world)
        .unwrap()
        .get_file_name();

    let type_registry = world.resource::<AppTypeRegistry>().read();

    let type_registry = type_registry.deref();

    let scene_builder = DynamicWorldBuilder::from_world(world, type_registry);

    let scene = scene_builder
        .deny_all_components()
        .allow_component::<Brick>()
        .allow_component::<Transform>()
        .extract_entities(entities.into_iter())
        .build();

    let ron = scene.serialize(&type_registry).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/{}", level_path))
                .and_then(|mut file| file.write(ron.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
}
