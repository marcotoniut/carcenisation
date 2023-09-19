use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use leafwing_input_manager::prelude::ActionState;
use seldom_pixel::prelude::*;

use crate::{
    globals::{HUD_HEIGHT, SCREEN_RESOLUTION},
    GBInput,
};

use super::{bundles::*, components::*, resources::*};
use super::{crosshair::CrosshairSettings, resources::AttackTimer};

pub fn spawn_player(
    mut commands: Commands,
    mut assets_sprite: PxAssets<PxSprite>,
    crosshair_settings: Res<CrosshairSettings>,
) {
    // if let Ok((entity, _)) = stage_query.get_single() {
    //     commands.entity(entity).despawn_recursive();
    // }
    commands.spawn(make_player_bundle(&mut assets_sprite, crosshair_settings));
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn confine_player_movement(mut player_query: Query<&mut PxSubPosition, With<Player>>) {
    if let Ok(mut position) = player_query.get_single_mut() {
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = SCREEN_RESOLUTION.x as f32 - half_player_size;
        let y_min = HUD_HEIGHT as f32 + half_player_size;
        let y_max = SCREEN_RESOLUTION.y as f32 - half_player_size;

        let mut translation = position.0;

        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        position.0 = translation;
    }
}

pub fn player_movement(
    gb_input_query: Query<&ActionState<GBInput>>,
    mut query: Query<(&mut PxSubPosition, &Player)>,
    time: Res<Time>,
) {
    let gb_input = gb_input_query.single();
    for (mut position, _) in &mut query {
        let mut direction = Vec2::new(
            (gb_input.pressed(GBInput::Right) as i32 - gb_input.pressed(GBInput::Left) as i32)
                as f32,
            (gb_input.pressed(GBInput::Up) as i32 - gb_input.pressed(GBInput::Down) as i32) as f32,
        );

        if direction.length() > 0.0 {
            direction = direction.normalize();
            position.0 += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn setup_weapon_recoil_timer(mut timer: ResMut<AttackTimer>) {
    timer.timer.pause();
}

pub fn tick_weapon_recoil_timer(mut timer: ResMut<AttackTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn check_weapon_recoil_timer(
    timer: ResMut<AttackTimer>,
    // event to attack?
    // mut event_writer: EventWriter<StageActionTrigger>,
) {
    if timer.timer.finished() {
        // event_writer.send(StageActionTrigger {});
    }
}

pub fn detect_player_attack(
    mut commands: Commands,
    mut asset_server: PxAssets<PxSprite>,
    mut timer: ResMut<AttackTimer>,
    gb_input_query: Query<&ActionState<GBInput>>,
    player_attack_query: Query<&PlayerAttack>,
    player_query: Query<&PxSubPosition, With<Player>>,
) {
    if let None = player_attack_query.iter().next() {
        let position = player_query.get_single().unwrap();
        let gb_input = gb_input_query.get_single().unwrap();

        if gb_input.just_pressed(GBInput::A) {
            timer.timer.set_duration(Duration::from_secs_f32(0.8));
            let player_attack = PlayerAttack {
                position: position.0.clone(),
                weapon: Weapon::Pincer,
            };

            let bundle = make_player_attack_bundle(&mut asset_server, player_attack);
            commands.spawn(bundle);
        } else if gb_input.just_pressed(GBInput::B) {
            timer.timer.set_duration(Duration::from_secs_f32(0.08));
            let player_attack = PlayerAttack {
                position: position.0.clone(),
                weapon: Weapon::Gun,
            };

            let bundle = make_player_attack_bundle(&mut asset_server, player_attack);
            commands.spawn(bundle);
        }
        timer.timer.reset();
        timer.timer.unpause();
    }
}

pub fn setup_attack_timer(mut timer: ResMut<AttackTimer>) {
    timer.timer.pause();
}

pub fn tick_attack_timer(mut timer: ResMut<AttackTimer>, time: Res<Time>) {
    timer.timer.tick(time.delta());
}

pub fn check_attack_timer(
    mut commands: Commands,
    timer: ResMut<AttackTimer>,
    player_attack_query: Query<(Entity, &PlayerAttack)>,
    // event to attack?
    // mut event_writer: EventWriter<StageActionTrigger>,
) {
    if timer.timer.finished() {
        for (entity, _) in &mut player_attack_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

// pub fn attack(
//     gb_input_query: Query<&ActionState<GBInput>>,
//     mut query: Query<(&mut PxSubPosition, &Player)>,
//     time: Res<Time>,
// ) {
//     let gb_input = gb_input_query.single();
//     for (mut position, _) in &mut query {
//         let mut direction = Vec2::new(
//             (gb_input.pressed(GBInput::Right) as i32 - gb_input.pressed(GBInput::Left) as i32)
//                 as f32,
//             (gb_input.pressed(GBInput::Up) as i32 - gb_input.pressed(GBInput::Down) as i32) as f32,
//         );

//         if direction.length() > 0.0 {
//             direction = direction.normalize();
//             position.0 += direction * PLAYER_SPEED * time.delta_seconds();
//         }
//     }
// }

// NOTE: Keeping as comment for quick reference
// pub fn enemy_hit_player(
//     mut commands: Commands,
//     mut game_over_event_writer: EventWriter<GameOver>,
//     mut player_query: Query<(Entity, &PxSubPosition), With<Player>>,
//     enemy_query: Query<&PxSubPosition, With<Enemy>>,
//     asset_server: Res<AssetServer>,
//     score: Res<Score>,
// ) {
//     if let Ok((player_entity, player_position)) = player_query.get_single_mut() {
//         for enemy_position in enemy_query.iter() {
//             let distance = player_position.0.distance(enemy_position.0);

//             if distance < (PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0) {
//                 commands.entity(player_entity).despawn();

//                 let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
//                 commands.spawn(AudioBundle {
//                     source: sound_effect,
//                     settings: PlaybackSettings {
//                         mode: PlaybackMode::Despawn,
//                         volume: Volume::new_relative(0.02),
//                         ..default()
//                     },
//                     ..default()
//                 });

//                 println!("Enemy hit player! Game over!");
//                 game_over_event_writer.send(GameOver { score: score.value });
//             }
//         }
//     }
// }
