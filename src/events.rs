use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Event)]
pub struct TogglePausedState;

#[derive(Event, Clone)]
pub struct SpawnBall{
    pub transform: Transform,
    pub velocity: LinearVelocity,
}