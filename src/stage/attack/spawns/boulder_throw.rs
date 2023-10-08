use bevy::prelude::*;
use seldom_pixel::{
    prelude::{PxAssets, PxSubPosition},
    sprite::PxSprite,
};

use crate::{
    plugins::movement::linear::components::{
        LinearAcceleration, LinearMovementBundle, LinearSpeed, LinearTargetPosition,
        TargetingPositionX, TargetingPositionY, TargetingPositionZ,
    },
    stage::{
        attack::{
            components::{
                bundles::make_hovering_attack_animation_bundle, EnemyAttack,
                EnemyHoveringAttackType,
            },
            data::boulder_throw::{
                BOULDER_THROW_ATTACK_DAMAGE, BOULDER_THROW_ATTACK_DEPTH_SPEED,
                BOULDER_THROW_ATTACK_LINE_Y_ACCELERATION, BOULDER_THROW_ATTACK_RANDOMNESS,
            },
        },
        components::{
            damage::InflictsDamage,
            interactive::{Flickerer, Health, Hittable},
            placement::Depth,
        },
        enemy::components::*,
        player::components::PLAYER_DEPTH,
        resources::StageTime,
    },
};

pub fn spawn_boulder_throw_attack(
    commands: &mut Commands,
    assets_sprite: &mut PxAssets<PxSprite>,
    stage_time: &Res<StageTime>,
    target_pos: Vec2,
    current_pos: Vec2,
    depth: &Depth,
) {
    let attack_type = EnemyHoveringAttackType::BoulderThrow;
    let target_pos = target_pos
        + Vec2::new(
            (1. - rand::random::<f32>()) * BOULDER_THROW_ATTACK_RANDOMNESS,
            (1. - rand::random::<f32>()) * BOULDER_THROW_ATTACK_RANDOMNESS,
        );

    let animation_bundle =
        make_hovering_attack_animation_bundle(assets_sprite, &attack_type, depth.clone());

    let mut attacking = EnemyTardigradeAttacking {
        attack: true,
        last_attack_started: stage_time.elapsed,
    };

    let target_depth = PLAYER_DEPTH + 1.;

    let speed_z = BOULDER_THROW_ATTACK_DEPTH_SPEED;
    let t = (target_depth - depth.0 as f32) / speed_z;

    let d = target_pos - current_pos;

    let speed_x = d.x / t;

    // TODO: remember that boulder throws in outter space wouldn't have as much gravity, if any at all
    let value = d.y - 0.5 * BOULDER_THROW_ATTACK_LINE_Y_ACCELERATION * t.powi(2);
    let speed_y = if value / t >= 0.0 { value / t } else { 0.0 };

    let movement_bundle = (
        TargetingPositionX::new(current_pos.x),
        LinearSpeed::<StageTime, TargetingPositionX>::new(speed_x),
        TargetingPositionY::new(current_pos.y),
        LinearSpeed::<StageTime, TargetingPositionY>::new(speed_y),
        LinearAcceleration::<StageTime, TargetingPositionY>::new(
            BOULDER_THROW_ATTACK_LINE_Y_ACCELERATION,
        ),
        LinearMovementBundle::<StageTime, TargetingPositionZ>::new(
            depth.0.clone() as f32,
            target_depth,
            BOULDER_THROW_ATTACK_DEPTH_SPEED,
        ),
    );

    attacking.attack = attacking.attack.clone();
    attacking.last_attack_started = attacking.last_attack_started.clone();

    commands
        .spawn((
            Name::new(format!("Attack - {}", attack_type.get_name())),
            EnemyAttack,
            EnemyHoveringAttackType::BoulderThrow,
            depth.clone(),
            TargetingPositionZ(depth.0.clone() as f32),
            InflictsDamage(BOULDER_THROW_ATTACK_DAMAGE),
            PxSubPosition(current_pos),
            Flickerer,
            Hittable,
            Health(100),
        ))
        .insert(movement_bundle)
        .insert(animation_bundle);
}