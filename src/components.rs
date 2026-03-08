use bevy::prelude::*;

#[derive(Component)]
#[require(Camera, Camera2d)]
pub struct MyCamera;

#[derive(Component)]
pub struct PlayablePlane;

#[derive(Component)]
pub struct Ball{
    pub velocity: Vec2
}