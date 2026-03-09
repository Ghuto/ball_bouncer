use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

#[derive(Event)]
pub struct TogglePausedState;

#[derive(Event, Clone)]
pub struct SpawnBall{
    pub transform: Transform,
    pub velocity: Velocity,
}