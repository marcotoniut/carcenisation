pub mod bundles;
pub mod components;
pub mod data;
pub mod enemy;
pub mod events;
pub mod pickup;
pub mod player;
pub mod resources;
pub mod score;
pub mod star;
pub mod systems;
pub mod ui;

use bevy::prelude::*;

use self::{
    enemy::EnemyPlugin,
    events::*,
    pickup::systems::health::pickup_health,
    player::PlayerPlugin,
    resources::{StageActionTimer, StageProgress},
    score::{components::Score, ScorePlugin},
    systems::{spawn::read_stage_spawn_trigger, *},
    ui::{pause_menu::pause_menu_renderer, StageUiPlugin},
};
use crate::{events::*, AppState};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct LoadingSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct BuildingSystemSet;

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<StageState>()
            .add_event::<GameOver>()
            .add_event::<StageStepTrigger>()
            .add_event::<StageSpawnTrigger>()
            .init_resource::<StageActionTimer>()
            .init_resource::<Score>()
            .init_resource::<StageProgress>()
            .add_plugins(EnemyPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(ScorePlugin)
            .add_plugins(StageUiPlugin)
            // .add_plugins(StarPlugin)
            .add_systems(PostStartup, setup_stage.in_set(LoadingSystemSet))
            .add_systems(
                Update,
                spawn_current_stage_bundle.run_if(in_state(GameState::Loading)),
            )
            .add_systems(
                Update,
                (
                    update_stage,
                    (
                        increment_elapsed,
                        tick_stage_step_timer,
                        read_stage_step_trigger,
                        read_stage_spawn_trigger,
                        check_stage_step_timer,
                        check_staged_cleared,
                        pickup_health,
                    )
                        .run_if(in_state(StageState::Running)),
                )
                    .run_if(in_state(GameState::Running)),
            )
            // .add_systems(Update, run_timer)
            .add_systems(Update, toggle_game.run_if(in_state(AppState::Game)))
            .add_systems(Update, pause_menu_renderer.run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(AppState::Game), resume_game);
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    Running,
    Paused,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum StageState {
    #[default]
    Initial,
    Running,
    Clear,
    Cleared,
}
