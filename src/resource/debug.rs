use crate::plugins::movement::structs::MovementDirection;
use crate::stage::data::*;
use bevy::prelude::*;

use lazy_static::lazy_static;

const OBJECT_FIBERTREE_Y: f32 = 13.;

lazy_static! {
    pub static ref STAGE_DEBUG_DATA: StageData = StageData {
        name: "Debug".to_string(),
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
        StageSpawn::Object(ObjectSpawn::fibertree_base(30., OBJECT_FIBERTREE_Y)),
        StageSpawn::Object(ObjectSpawn::fibertree_base(180., OBJECT_FIBERTREE_Y)),
        StageSpawn::Object(ObjectSpawn::bench_big_base(20., 65.)),
        StageSpawn::Object(ObjectSpawn::bench_big_base(200., 60.)),
        StageSpawn::Destructible(
            DestructibleSpawn::lamp_base(30., 0.).drops(ContainerSpawn::Enemy(
                EnemySpawn::mosquito_base()
                    .set_coordinates(Vec2::new(60., 100.))
                    .set_elapsed(0.4)
                    .set_steps_vec(vec![
                        EnemyStep::Idle { duration: 1. },
                        EnemyStep::Attack { duration: 1. },
                    ]),
            )),
        ),
        StageSpawn::Destructible(
            DestructibleSpawn::lamp_base(20., 0.)
                .drops(ContainerSpawn::Pickup(PickupSpawn::big_healthpack_base())),
        ),
    ]
}

pub fn make_steps() -> Vec<StageStep> {
    vec![
        StageStep::movement_base(0.0, 0.0),
        // StageStep::Cinematic {
        //     cinematic: INTRO_ANIMATIC_0.clone(),
        // },
        // StageStep::Cinematic {
        //     cinematic: INTRO_ANIMATIC_1.clone(),
        // },
        // StageStep::Cinematic {
        //     cinematic: INTRO_ANIMATIC_2.clone(),
        // },
        // StageStep::Cinematic {
        //     cinematic: INTRO_ANIMATIC_3.clone(),
        // },
        // StageStep::Cinematic {
        //     cinematic: INTRO_ANIMATIC_4.clone(),
        // },
        StageStep::stop_base()
            .set_max_duration(0.1)
            .set_resume_conditions(vec![]),
        StageStep::stop_base()
            .set_max_duration(30.0)
            .set_resume_conditions(vec![StageActionResumeCondition::KillAll])
            .add_spawns(vec![StageSpawn::Enemy(
                EnemySpawn::mosquito_base().set_coordinates(Vec2::new(70.0, 70.0)),
            )]),
        StageStep::movement_base(100.0, 0.0).add_spawns(vec![
            StageSpawn::Enemy(
                EnemySpawn::tardigrade_base()
                    .set_coordinates(Vec2::new(60.0, 100.0))
                    .set_elapsed(5.4)
                    .set_steps_vec(vec![
                        EnemyStep::Circle {
                            duration: 4.0,
                            radius: 10.0,
                            direction: MovementDirection::Negative,
                        },
                        EnemyStep::LinearMovement {
                            coordinates: Vec2::new(50.0, 0.0),
                            attacking: true,
                            speed: 5.0,
                        },
                        EnemyStep::Idle { duration: 1.0 },
                        EnemyStep::Attack { duration: 1.0 },
                        EnemyStep::LinearMovement {
                            coordinates: Vec2::new(10.0, 0.0),
                            attacking: true,
                            speed: 3.0,
                        },
                    ])
                    .drops(ContainerSpawn::Pickup(PickupSpawn::small_healthpack_base())),
            ),
            StageSpawn::Enemy(
                EnemySpawn::mosquito_base()
                    .set_coordinates(Vec2::new(120.0, 100.0))
                    .set_elapsed(5.1)
                    .drops(ContainerSpawn::Pickup(PickupSpawn::big_healthpack_base())),
            ),
            StageSpawn::Enemy(
                EnemySpawn::mosquito_base()
                    .set_coordinates(Vec2::new(130.0, 70.0))
                    .set_elapsed(5.8)
                    .drops(ContainerSpawn::Pickup(PickupSpawn::big_healthpack_base())),
            ),
        ]),
    ]
}
