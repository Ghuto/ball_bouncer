use bevy::prelude::*;

#[derive(Component, Clone, Default)]
#[require(Camera2d)]
pub struct MyCamera;

pub fn camera_scene() -> impl Scene {
    bsn! {
        #MyCamera
        MyCamera
    }
}
