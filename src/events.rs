use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::components::Ball;

#[derive(Event)]
pub struct TogglePausedState;

#[derive(Event, Clone)]
pub struct SpawnBall{
    pub transform: Transform,
    pub velocity: Velocity,
}