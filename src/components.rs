use bevy::prelude::*;

#[derive(Component)]
#[require(Camera2d)]
pub struct MyCamera;

#[derive(Component)]
pub struct PlayablePlane;

#[derive(Component,Clone)]
pub struct Ball;