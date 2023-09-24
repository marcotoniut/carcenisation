use crate::cinemachine::scene_intro::INTRO_ANIMATIC;
use crate::resource::CAMERA_BASE_SPEED;
use crate::stage::data::{
    ContainerSpawn, DestructibleSpawn, EnemySpawn, MovementDirection, ObjectSpawn, ObjectType,
    PickupSpawn, SkyboxData, StageData, StageStep,
};
use crate::stage::data::{
    DestructibleType, EnemyStep, EnemyType, PickupType, StageActionResumeCondition, StageSpawn,
};
use bevy::prelude::*;

use lazy_static::lazy_static;

const OBJECT_FIBERTREE_Y: f32 = 13.;

lazy_static! {
    pub static ref STAGE_PARK_DATA: StageData = StageData {
        name: "Park".to_string(),
        music_path: "audio/music/stage_1.ogg".to_string(),
        background_path: "backgrounds/rugpark/background.png".to_string(),
        skybox: SkyboxData {
            path: "backgrounds/rugpark/skybox.png".to_string(),
            frames: 2,
        },
        start_coordinates: Some(Vec2::new(0.0, 0.0)),
        spawns: make_spawns(),
        steps: make_steps(),
    };
}

pub fn make_spawns() -> Vec<StageSpawn> {
    vec![
        StageSpawn::Object(ObjectSpawn {
            object_type: ObjectType::Fibertree,
            coordinates: Vec2 {
                x: 30.,
                y: OBJECT_FIBERTREE_Y,
            },
        }),
        StageSpawn::Object(ObjectSpawn {
            object_type: ObjectType::Fibertree,
            coordinates: Vec2 {
                x: 180.,
                y: OBJECT_FIBERTREE_Y,
            },
        }),
        StageSpawn::Object(ObjectSpawn {
            object_type: ObjectType::Fibertree,
            coordinates: Vec2 {
                x: 320.,
                y: OBJECT_FIBERTREE_Y,
            },
        }),
        StageSpawn::Object(ObjectSpawn {
            object_type: ObjectType::BenchBig,
            coordinates: Vec2 { x: 20., y: 65. },
        }),
        StageSpawn::Object(ObjectSpawn {
            object_type: ObjectType::BenchBig,
            coordinates: Vec2 { x: 200., y: 60. },
        }),
        StageSpawn::Object(ObjectSpawn {
            object_type: ObjectType::BenchSmall,
            coordinates: Vec2 { x: 100., y: 65. },
        }),
        StageSpawn::Destructible(DestructibleSpawn {
            coordinates: Vec2 { x: 30., y: 0. },
            contains: Some(Box::new(ContainerSpawn::Enemy(EnemySpawn {
                enemy_type: EnemyType::Mosquito,
                coordinates: Vec2 { x: 60., y: 100. },
                base_speed: 5.0,
                elapsed: 0.4 / CAMERA_BASE_SPEED,
                steps: vec![
                    EnemyStep::Movement {
                        coordinates: Vec2 { x: 50., y: 0. },
                        attacking: true,
                        speed: 5.0,
                    },
                    EnemyStep::Idle { duration: 1. },
                    EnemyStep::Attack { duration: 1. },
                    EnemyStep::Movement {
                        coordinates: Vec2 { x: 10., y: 0. },
                        attacking: true,
                        speed: 3.,
                    },
                    EnemyStep::Circle { duration: 4. },
                ],
                direction: MovementDirection::Left,
                radius: 15.,
                time_offset: 0.5,
                contains: None,
            }))),
            destructible_type: DestructibleType::Lamp,
        }),
        StageSpawn::Destructible(DestructibleSpawn {
            coordinates: Vec2 { x: 20.0, y: 0.0 },
            contains: Some(Box::new(ContainerSpawn::Pickup(PickupSpawn {
                pickup_type: PickupType::BigHealthpack,
                coordinates: Vec2 { x: 30.0, y: 10.0 },
                elapsed: 0.0 / CAMERA_BASE_SPEED,
            }))),
            destructible_type: DestructibleType::Lamp,
        }),
        StageSpawn::Pickup(PickupSpawn {
            pickup_type: PickupType::BigHealthpack,
            coordinates: Vec2 { x: 30.0, y: 10.0 },
            elapsed: 0.0 / CAMERA_BASE_SPEED,
        }),
    ]
}

pub fn make_steps() -> Vec<StageStep> {
    vec![
        // StageStep::Cinematic {
        //     resume_conditions: Some(vec![StageActionResumeCondition::CinematicEnd]),
        //     max_duration: Some(5),
        //     cinematic: INTRO_ANIMATIC.clone(),
        // },
        StageStep::Movement {
            coordinates: Vec2 { x: 50.0, y: 0.0 },
            base_speed: CAMERA_BASE_SPEED,
            spawns: vec![
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 60.0, y: 100.0 },
                    base_speed: 5.0,
                    elapsed: 5.4 / CAMERA_BASE_SPEED,

                    direction: MovementDirection::Right,
                    radius: 15.,
                    time_offset: 1.5,
                    steps: vec![
                        EnemyStep::Movement {
                            coordinates: Vec2 { x: 50.0, y: 0.0 },
                            attacking: true,
                            speed: 5.0,
                        },
                        EnemyStep::Idle { duration: 1.0 },
                        EnemyStep::Attack { duration: 1.0 },
                        EnemyStep::Movement {
                            coordinates: Vec2 { x: 10.0, y: 0.0 },
                            attacking: true,
                            speed: 3.0,
                        },
                        EnemyStep::Circle { duration: 4.0 },
                    ],
                    contains: Some(Box::new(ContainerSpawn::Pickup(PickupSpawn {
                        pickup_type: PickupType::BigHealthpack,
                        ..default()
                    }))),
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 120.0, y: 100.0 },
                    base_speed: 5.0,
                    elapsed: 5.1 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Left,
                    radius: 25.,
                    time_offset: 3.5,

                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 130.0, y: 70.0 },
                    base_speed: 5.0,
                    elapsed: 5.8 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 10.,
                    time_offset: 2.5,

                    contains: None,
                }),
            ],
        },
        StageStep::Stop {
            resume_conditions: Some(vec![StageActionResumeCondition::KillAll]),
            max_duration: Some(30. / CAMERA_BASE_SPEED),
            spawns: vec![StageSpawn::Enemy(EnemySpawn {
                enemy_type: EnemyType::Mosquito,
                coordinates: Vec2 { x: 20.0, y: 0.0 },
                base_speed: CAMERA_BASE_SPEED,
                elapsed: 2.4 / CAMERA_BASE_SPEED,
                steps: vec![],
                direction: MovementDirection::Right,
                radius: 15.,
                time_offset: 3.5,

                contains: None,
            })],
        },
        //TEST down
        StageStep::Movement {
            coordinates: Vec2 { x: 10.0, y: 22.0 },
            base_speed: CAMERA_BASE_SPEED,
            spawns: vec![
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 40.0, y: 70.0 },
                    base_speed: 4.0,
                    elapsed: 6.8 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 10.,
                    time_offset: 3.5,

                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 90.0, y: 30.0 },
                    base_speed: 2.0,
                    elapsed: 22.8 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 10.,
                    time_offset: 2.5,

                    contains: None,
                }),
                StageSpawn::Pickup(PickupSpawn {
                    pickup_type: PickupType::BigHealthpack,
                    coordinates: Vec2 { x: 38.0, y: 68.0 },
                    elapsed: 2.0 / CAMERA_BASE_SPEED,
                }),
            ],
        },
        //TEST up
        StageStep::Stop {
            resume_conditions: Some(vec![]),
            max_duration: Some(40. / CAMERA_BASE_SPEED),
            spawns: vec![],
        },
        StageStep::Movement {
            coordinates: Vec2 { x: 120.0, y: 20.0 },
            base_speed: 4.0,
            spawns: vec![
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 60.0, y: 80.0 },
                    base_speed: 5.0,
                    elapsed: 8.4 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 15.0,
                    time_offset: 0.5,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 100.0, y: 60.0 },
                    base_speed: 5.0,
                    elapsed: 6.1 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Left,
                    radius: 25.0,
                    time_offset: 3.0,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 140.0, y: 80.0 },
                    base_speed: 5.0,
                    elapsed: 7.4 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 15.0,
                    time_offset: 1.0,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 180.0, y: 60.0 },
                    base_speed: 5.0,
                    elapsed: 5.1 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Left,
                    radius: 25.0,
                    time_offset: 2.0,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 220.0, y: 80.0 },
                    base_speed: 5.0,
                    elapsed: 6.4 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 15.0,
                    time_offset: 0.5,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 260.0, y: 60.0 },
                    base_speed: 5.0,
                    elapsed: 8.1 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Left,
                    radius: 25.0,
                    time_offset: 3.0,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 300.0, y: 80.0 },
                    base_speed: 5.0,
                    elapsed: 7.4 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 15.0,
                    time_offset: 1.0,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 340.0, y: 60.0 },
                    base_speed: 5.0,
                    elapsed: 6.1 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Left,
                    radius: 25.0,
                    time_offset: 2.0,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 380.0, y: 80.0 },
                    base_speed: 5.0,
                    elapsed: 7.4 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Right,
                    radius: 15.0,
                    time_offset: 0.5,
                    contains: None,
                }),
                StageSpawn::Enemy(EnemySpawn {
                    enemy_type: EnemyType::Mosquito,
                    coordinates: Vec2 { x: 420.0, y: 60.0 },
                    base_speed: 5.0,
                    elapsed: 8.1 / CAMERA_BASE_SPEED,
                    steps: vec![],
                    direction: MovementDirection::Left,
                    radius: 25.0,
                    time_offset: 3.0,
                    contains: None,
                }),
            ],
        },
        StageStep::Movement {
            coordinates: Vec2 { x: 0.0, y: 0.0 },
            base_speed: 2.0,
            spawns: vec![],
        },
        //TODO
        StageStep::Stop {
            resume_conditions: Some(vec![
                StageActionResumeCondition::KillAll,
                StageActionResumeCondition::KillBoss,
            ]),
            max_duration: None,
            spawns: vec![],
        },
    ]
}
