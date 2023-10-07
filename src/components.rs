use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct DespawnMark;

#[derive(Component)]
pub struct Music {}

#[derive(Component)]
pub struct DelayedDespawnOnPxAnimationFinished(pub Duration);

impl DelayedDespawnOnPxAnimationFinished {
    pub fn from_secs_f32(secs: f32) -> Self {
        Self(Duration::from_secs_f32(secs))
    }
}

#[derive(Component)]
pub struct DespawnAfterDelay {
    pub elapsed: Duration,
    pub duration: Duration,
}
