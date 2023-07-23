use bevy::prelude::*;

// TODO should come from this module?
use crate::{events::GameOver, AppState};

use self::systems::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
pub mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_event::<GameOver>()
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_plugins(enemy::EnemyPlugin)
            .add_plugins(player::PlayerPlugin)
            .add_plugins(score::ScorePlugin)
            .add_plugins(star::StarPlugin)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(AppState::Game), resume_simulation);
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
