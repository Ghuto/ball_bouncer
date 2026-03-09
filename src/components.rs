use bevy::prelude::*;

#[derive(Component)]
#[require(Camera, Camera2d)]
pub struct MyCamera;

#[derive(Component)]
pub struct PlayablePlane;

#[derive(Component,Clone)]
pub struct Ball{
    pub velocity: Vec2
}