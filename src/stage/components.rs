use bevy::prelude::*;

use super::data::ContainerSpawn;

#[derive(Component)]
pub struct Stage {}

#[derive(Component, Debug, Clone, Copy)]
pub struct Depth(pub usize);

#[derive(Component)]
pub struct Destructible {}

#[derive(Component)]
pub struct Object {}

// TODO should go in UI
#[derive(Component, Debug, Clone)]
pub struct StageClearedText {}

#[derive(Component, Debug, Clone)]
pub struct DepthSpeed(pub f32);

#[derive(Component, Debug, Clone)]
pub struct DepthProgress(pub f32);

#[derive(Component, Debug, Clone)]
pub struct DepthReached;

#[derive(Component, Debug, Clone)]
pub struct TargetDepth(pub usize);

#[derive(Component, Debug, Clone)]
pub struct LineSpeed(pub Vec2);

#[derive(Component, Debug, Clone)]
pub struct TargetPosition(pub Vec2);

#[derive(Component, Debug, Clone)]
pub struct TargetXReached;

#[derive(Component, Debug, Clone)]
pub struct TargetYReached;

#[derive(Component, Debug, Clone)]
pub struct TargetReached;

#[derive(Component, Debug, Clone)]
pub enum Collision {
    Box(Vec2),
    Circle(f32),
}

// TODO impl more complex collision algorithm

#[derive(Component, Debug, Clone)]
pub struct Health(pub u32);

#[derive(Component, Debug, Clone)]
pub struct Damage(pub u32);

#[derive(Component, Debug, Clone)]
pub struct Hittable {}

// TODO? critical kill
#[derive(Component, Debug, Clone)]
pub struct Dead;

#[derive(Component, Debug, Clone)]
pub struct SpawnDrop {
    pub contains: ContainerSpawn,
    pub entity: Entity,
}
