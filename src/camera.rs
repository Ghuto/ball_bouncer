use bevy::prelude::*;

#[derive(Component)]
#[require(Camera2d)]
pub struct MyCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(MyCamera);
}
