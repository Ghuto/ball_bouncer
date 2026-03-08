use bevy::prelude::*;

#[derive(Component)]
#[require(Camera, Camera2d)]
pub struct MyCamera;

#[derive(Component)]
pub struct PlayablePlane;