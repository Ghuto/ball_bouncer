use bevy::prelude::*;

use crate::components::Ball;

#[derive(Event)]
pub struct TogglePausedState;

#[derive(Event, Clone)]
pub struct SpawnBall{
    pub transform: Transform,
    pub ball: Ball,
}