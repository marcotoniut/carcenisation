pub mod camera;
pub mod damage;
pub mod movement;
pub mod spawn;

use self::spawn::{spawn_destructible, spawn_enemy, spawn_object, spawn_pickup};
use super::{
    bundles::*,
    components::{
        interactive::{Dead, Destructible, Object},
        Stage,
    },
    data::*,
    enemy::components::Enemy,
    events::{StageClearedEvent, StageGameOverEvent, StageSpawnEvent, StageStepEvent},
    player::components::Player,
    resources::{StageActionTimer, StageProgress, StageTime},
    GameState, StageState,
};
use crate::{
    cinemachine::cinemachine::CinemachineScene,
    components::{DespawnMark, Music},
    globals::{mark_for_despawn_by_component_query, DEBUG_STAGESTEP},
    resource::{asteroid::STAGE_ASTEROID_DATA, debug::STAGE_DEBUG_DATA, park::STAGE_PARK_DATA},
    systems::{audio::VolumeSettings, camera::CameraPos, spawn::spawn_music},
    GBInput,
};
use bevy::{audio::PlaybackMode, prelude::*};
use leafwing_input_manager::prelude::ActionState;
use seldom_pixel::{
    prelude::{PxAssets, PxCamera, PxSubPosition},
    sprite::PxSprite,
};
use std::{ops::Sub, time::Duration};

pub fn tick_stage_time(mut stage_time: ResMut<StageTime>, time: Res<Time>) {
    let delta = time.delta();
    stage_time.delta = delta;
    stage_time.elapsed += delta;
}

pub fn pause_game(mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Paused);
}

pub fn resume_game(mut game_state_next_state: ResMut<NextState<GameState>>) {
    game_state_next_state.set(GameState::Running);
}

pub fn toggle_game(
    gb_input_query: Query<&ActionState<GBInput>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let gb_input = gb_input_query.single();
    if gb_input.just_pressed(GBInput::Start) {
        if state.get().to_owned() == GameState::Running {
            next_state.set(GameState::Paused);
            info!("Game Paused.");
        } else {
            next_state.set(GameState::Running);
            info!("Game Running.");
        }
    }
}

#[derive(Resource)]
pub struct StageRawData {
    stage_data: StageData,
}

pub fn setup_stage(
    mut commands: Commands,
    mut assets_sprite: PxAssets<PxSprite>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&PxSubPosition, With<CameraPos>>,
    volume_settings: Res<VolumeSettings>,
) {
    let camera_pos = camera_query.get_single().unwrap();

    let stage_data = STAGE_DEBUG_DATA.clone();
    // let stage_data = STAGE_PARK_DATA.clone();

    for spawn in &stage_data.spawns {
        match spawn {
            StageSpawn::Destructible(spawn) => {
                spawn_destructible(&mut commands, &mut assets_sprite, spawn);
            }
            StageSpawn::Enemy(spawn) => {
                spawn_enemy(&mut commands, camera_pos.0, spawn);
            }
            StageSpawn::Object(spawn) => {
                spawn_object(&mut commands, &mut assets_sprite, spawn);
            }
            StageSpawn::Pickup(spawn) => {
                spawn_pickup(&mut commands, &mut assets_sprite, Vec2::ZERO, spawn);
            }
        }
    }

    spawn_music(
        &mut commands,
        &asset_server,
        &volume_settings,
        stage_data.music_path.clone(),
        PlaybackMode::Loop,
    );

    commands.insert_resource(StageRawData { stage_data });
}

pub fn spawn_current_stage_bundle(
    mut commands: Commands,
    mut assets_sprite: PxAssets<PxSprite>,
    mut state: ResMut<NextState<GameState>>,
    stage_data_raw: Res<StageRawData>,
) {
    let stage = &stage_data_raw.stage_data;
    commands
        .spawn((Stage, Name::new("Stage")))
        .with_children(|parent| {
            let background_bundle =
                make_background_bundle(&mut assets_sprite, stage.background_path.clone());
            parent.spawn(background_bundle);

            let skybox_bundle = make_skybox_bundle(&mut assets_sprite, stage.skybox.clone());
            parent.spawn(skybox_bundle);
        });

    state.set(GameState::Running);
}

// TODO Probably can do without this now
/**
 *  @deprecate in favor of the stage_time
*/
pub fn increment_elapsed(mut progress: ResMut<StageProgress>, time: Res<Time>) {
    let delta = time.delta_seconds();
    progress.elapsed += delta;
    progress.step_elapsed += delta;
    progress.spawn_step_elapsed += delta;
}

pub fn tick_stage_step_timer(mut timer: ResMut<StageActionTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn check_stage_step_timer(
    timer: Res<StageActionTimer>,
    mut event_writer: EventWriter<StageStepEvent>,
) {
    if timer.timer.finished() {
        event_writer.send(StageStepEvent {});
    }
}

pub fn update_stage(
    mut commands: Commands,
    state: Res<State<StageState>>,
    stage_query: Query<(Entity, &Stage)>,
    mut next_state: ResMut<NextState<StageState>>,
    mut camera_pos_query: Query<&mut PxSubPosition, With<CameraPos>>,
    mut camera: ResMut<PxCamera>,
    mut spawn_event_writer: EventWriter<StageSpawnEvent>,
    mut step_event_writer: EventWriter<StageStepEvent>,
    mut stage_progress: ResMut<StageProgress>,
    time: Res<Time>,
    stage_data_raw: Res<StageRawData>,
) {
    match state.to_owned() {
        StageState::Initial => {
            next_state.set(StageState::Running);
        }
        StageState::Running => {
            let stage = &stage_data_raw.stage_data;
            if let Some(action) = stage.steps.get(stage_progress.step) {
                if DEBUG_STAGESTEP {
                    let curr_action = match action {
                        StageStep::Movement { .. } => "movement".to_string(),
                        StageStep::Stop { .. } => "stop".to_string(),
                        StageStep::Cinematic { .. } => "cinematic".to_string(),
                    };

                    info!("curr action: {}", curr_action);
                }

                let mut spawns_val = None;
                match action {
                    StageStep::Movement {
                        coordinates,
                        spawns,
                        ..
                    } => {
                        let mut camera_pos = camera_pos_query.single_mut();
                        let direction = coordinates.sub(camera_pos.0).normalize();

                        **camera_pos += time.delta_seconds() * action.speed() * direction;

                        if direction.x.signum() != (coordinates.x - camera_pos.0.x).signum() {
                            if DEBUG_STAGESTEP {
                                warn!(
                                    "================>>>> movement complete? {}",
                                    direction.x.to_string()
                                );
                            }
                            *camera_pos = PxSubPosition(coordinates.clone());
                            step_event_writer.send(StageStepEvent {});
                        }

                        **camera = camera_pos.round().as_ivec2();

                        spawns_val = Some(spawns);
                    }
                    StageStep::Stop(StageStepStop {
                        max_duration,
                        spawns,
                        ..
                    }) => {
                        // TODO
                        if let Some(duration) = max_duration {
                        } else {
                            step_event_writer.send(StageStepEvent {});
                        }
                        spawns_val = Some(spawns);
                    }
                    StageStep::Cinematic { cinematic, .. } => {
                        let max_duration = Some(cinematic.clip.duration);

                        if let Some(duration) = max_duration {
                        } else {
                            step_event_writer.send(StageStepEvent {});
                        }
                    }
                }

                if let Some(spawns) = spawns_val {
                    let mut cloned_spawns = spawns.clone();

                    let mut i = 0;
                    while let Some(spawn) = cloned_spawns.first() {
                        if (stage_progress.spawn_step <= i) {
                            let elapsed = stage_progress.spawn_step_elapsed - spawn.get_elapsed();
                            if 0. <= elapsed {
                                stage_progress.spawn_step_elapsed -= spawn.get_elapsed();

                                spawn_event_writer.send(StageSpawnEvent {
                                    spawn: spawn.clone(),
                                });
                            } else {
                                break;
                            }
                        }
                        cloned_spawns.remove(0);
                        i += 1;
                    }
                    stage_progress.spawn_step = i;
                }
            }
        }
        StageState::Clear => {
            if let Ok((entity, _)) = stage_query.get_single() {
                commands.entity(entity).insert(DespawnMark);

                // TODO
                // commands.spawn(make_stage_cleared_bundle());
            }

            next_state.set(StageState::Cleared);
        }
        _ => {}
    }
}

pub fn check_staged_cleared(
    mut event_writer: EventWriter<StageClearedEvent>,
    stage_progress: Res<StageProgress>,
    stage_data_raw: Res<StageRawData>,
) {
    let stage = &stage_data_raw.stage_data;
    if stage_progress.step >= stage.steps.len() {
        event_writer.send(StageClearedEvent {});
    }
}

pub fn read_stage_cleared_trigger(
    mut commands: Commands,
    mut next_state: ResMut<NextState<StageState>>,
    mut event_reader: EventReader<StageClearedEvent>,
    destructible_query: Query<Entity, With<Destructible>>,
    enemy_query: Query<Entity, With<Enemy>>,
    music_query: Query<Entity, With<Music>>,
    object_query: Query<Entity, With<Object>>,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    volume_settings: Res<VolumeSettings>,
) {
    for _ in event_reader.iter() {
        mark_for_despawn_by_component_query(&mut commands, &destructible_query);
        mark_for_despawn_by_component_query(&mut commands, &enemy_query);
        mark_for_despawn_by_component_query(&mut commands, &music_query);
        mark_for_despawn_by_component_query(&mut commands, &object_query);
        mark_for_despawn_by_component_query(&mut commands, &player_query);

        spawn_music(
            &mut commands,
            &asset_server,
            &volume_settings,
            "audio/music/intro.ogg".to_string(),
            PlaybackMode::Despawn,
        );

        next_state.set(StageState::Cleared);
    }
}

pub fn check_stage_game_over(
    mut event_writer: EventWriter<StageGameOverEvent>,
    player_query: Query<&Player, Added<Dead>>,
) {
    if let Ok(_) = player_query.get_single() {
        event_writer.send(StageGameOverEvent {});
    }
}

pub fn read_stage_game_over_trigger(
    mut commands: Commands,
    mut next_state: ResMut<NextState<StageState>>,
    mut event_reader: EventReader<StageGameOverEvent>,
    destructible_query: Query<Entity, With<Destructible>>,
    enemy_query: Query<Entity, With<Enemy>>,
    music_query: Query<Entity, With<Music>>,
    object_query: Query<Entity, With<Object>>,
    player_query: Query<Entity, With<Player>>,
    asset_server: Res<AssetServer>,
    volume_settings: Res<VolumeSettings>,
) {
    for _ in event_reader.iter() {
        mark_for_despawn_by_component_query(&mut commands, &destructible_query);
        mark_for_despawn_by_component_query(&mut commands, &enemy_query);
        mark_for_despawn_by_component_query(&mut commands, &music_query);
        mark_for_despawn_by_component_query(&mut commands, &object_query);
        mark_for_despawn_by_component_query(&mut commands, &player_query);

        spawn_music(
            &mut commands,
            &asset_server,
            &volume_settings,
            "audio/music/game_over.ogg".to_string(),
            PlaybackMode::Despawn,
        );

        next_state.set(StageState::GameOver);
    }
}

pub fn read_stage_step_trigger(
    mut event_reader: EventReader<StageStepEvent>,
    mut stage_progress: ResMut<StageProgress>,
    mut stage_action_timer: ResMut<StageActionTimer>,
    stage_data_raw: Res<StageRawData>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut current_scene: ResMut<CinemachineScene>,
) {
    for _ in event_reader.iter() {
        stage_progress.step += 1;
        stage_progress.step_elapsed = 0.;
        stage_progress.spawn_step = 0;
        stage_progress.spawn_step_elapsed = 0.;

        let stage = &stage_data_raw.stage_data;
        if let Some(action) = stage.steps.get(stage_progress.step) {
            stage_action_timer.timer.pause();
            match action {
                StageStep::Movement { .. } => {
                    stage_action_timer.timer.reset();
                }
                StageStep::Stop(StageStepStop { max_duration, .. }) => {
                    if let Some(duration) = max_duration {
                        stage_action_timer.timer.reset();
                        stage_action_timer
                            .timer
                            .set_duration(Duration::from_secs_f32(duration.clone()));
                        stage_action_timer.timer.unpause();
                    }
                }
                StageStep::Cinematic { cinematic, .. } => {
                    let max_duration = Some(cinematic.clip.duration);

                    if let Some(duration) = max_duration {
                        stage_action_timer.timer.reset();
                        stage_action_timer.timer.set_duration(duration.clone());
                        stage_action_timer.timer.unpause();
                    }

                    current_scene.0 = Some(cinematic.clone());
                    game_state_next_state.set(GameState::Cutscene);
                }
            }
        }
    }
}
