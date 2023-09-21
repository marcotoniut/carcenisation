use bevy::{
    prelude::{Handle, Resource, Vec2},
    reflect::{TypePath, TypeUuid},
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub enum DestructibleType {
    Lamp,
    Window,
}

#[derive(Debug, Deserialize, Clone)]
pub enum PowerupType {
    Health,
}

#[derive(Debug, Deserialize, Clone)]
pub enum EnemyType {
    Mosquito,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum EnemyStep {
    Movement {
        coordinates: Vec2,
        attacking: bool,
        speed: f32,
    },
    Stop {
        duration: f32,
    },
    Attack {
        duration: f32,
    },
    CircleAround {
        duration: f32,
    },
}

fn default_zero() -> f32 {
    0.0
}

fn empty_vec<T: Clone>() -> Vec<T> {
    [].to_vec()
}

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum StageSpawn {
    Destructible {
        destructible_type: DestructibleType,
        coordinates: Vec2,
        #[serde(default = "default_zero")]
        elapsed: f32,
    },
    Powerup {
        powerup_type: PowerupType,
        coordinates: Vec2,
        #[serde(default = "default_zero")]
        elapsed: f32,
    },
    Enemy {
        enemy_type: EnemyType,
        coordinates: Vec2,
        base_speed: f32,
        #[serde(default = "default_zero")]
        elapsed: f32,
        #[serde(default = "empty_vec")]
        steps: Vec<EnemyStep>,
    },
}

impl StageSpawn {
    pub fn get_elapsed(&self) -> f32 {
        match self {
            StageSpawn::Destructible { elapsed, .. } => *elapsed,
            StageSpawn::Powerup { elapsed, .. } => *elapsed,
            StageSpawn::Enemy { elapsed, .. } => *elapsed,
        }
    }

    pub fn show_spawn_type(&self) -> String {
        match self {
            StageSpawn::Destructible {
                destructible_type, ..
            } => {
                format!("Destructible({:?})", destructible_type)
            }
            StageSpawn::Powerup { powerup_type, .. } => format!("Powerup({:?})", powerup_type),
            StageSpawn::Enemy { enemy_type, .. } => format!("Enemy({:?})", enemy_type),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum StageActionResumeCondition {
    KillAll,
    KillBoss,
}

fn default_base_speed() -> f32 {
    1.0
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum StageStep {
    Movement {
        coordinates: Vec2,
        #[serde(default = "default_base_speed")]
        base_speed: f32,
        #[serde(default = "empty_vec")]
        spawns: Vec<StageSpawn>,
    },
    Stop {
        resume_conditions: Option<Vec<StageActionResumeCondition>>,
        max_duration: Option<u64>,
        #[serde(default = "empty_vec")]
        spawns: Vec<StageSpawn>,
    },
}

#[derive(Deserialize, TypeUuid, TypePath, Clone, Debug)]
#[uuid = "c17075ed-7df0-4a51-b961-ce5270a8a934"]
pub struct StageData {
    pub name: String,
    pub background: String,
    pub skybox: Option<String>,
    pub start_coordinates: Option<Vec2>,
    #[serde(default = "empty_vec")]
    pub spawns: Vec<StageSpawn>,
    pub steps: Vec<StageStep>,
}