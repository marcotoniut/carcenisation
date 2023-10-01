use bevy::prelude::*;
use seldom_pixel::prelude::PxSubPosition;

use crate::stage::{
    components::placement::Speed,
    enemy::{
        components::{
            behavior::{BehaviorBundle, EnemyBehaviors, EnemyCurrentBehavior},
            CircleAround, Enemy,
        },
        resources::EnemyBehaviorTimer,
    },
    resources::StageTime,
};

pub fn check_no_behavior(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut EnemyBehaviors, &PxSubPosition, &Speed),
        (With<Enemy>, Without<EnemyCurrentBehavior>),
    >,
    stage_time: Res<StageTime>,
) {
    for (entity, mut behaviors, position, speed) in query.iter_mut() {
        let behavior = behaviors.next();

        let duration_o = behavior.get_duration_o();

        let current_behavior = EnemyCurrentBehavior {
            started: stage_time.elapsed,
            behavior,
        };

        let bundles = current_behavior.get_bundles(stage_time.elapsed, position, speed.0);
        match bundles {
            BehaviorBundle::Idle(bundles) => commands.entity(entity).insert(bundles),
            BehaviorBundle::LinearMovement(bundles) => commands.entity(entity).insert(bundles),
            BehaviorBundle::Attack(bundles) => commands.entity(entity).insert(bundles),
            BehaviorBundle::Circle(bundles) => commands.entity(entity).insert(bundles),
            BehaviorBundle::Jump(bundles) => commands.entity(entity).insert(bundles),
        };

        commands
            .entity(entity)
            .insert(current_behavior)
            .with_children(|parent| {
                if let Some(duration) = duration_o {
                    parent.spawn(EnemyBehaviorTimer::new(entity, duration));
                }
            });
    }
}

pub fn tick_enemy_behavior_timer(
    mut commands: Commands,
    mut query: Query<&mut EnemyBehaviorTimer>,
    stage_time: Res<StageTime>,
) {
    for mut behavior in query.iter_mut() {
        behavior.timer.tick(stage_time.delta);

        if behavior.finished() {
            commands
                .entity(behavior.entity)
                .remove::<EnemyCurrentBehavior>();
        }
    }
}

/**
 * could this be made into a generic?
 */
pub fn tied_components_enemy_current_behavior_circle_around(
    mut commands: Commands,
    query: Query<Entity, (With<CircleAround>, Without<EnemyCurrentBehavior>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).remove::<CircleAround>();
    }
}
