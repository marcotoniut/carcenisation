#![feature(step_trait)]

mod assets;
mod bevy_utils;
mod components;
mod core;
mod cutscene;
mod data;
mod debug;
mod game;
mod globals;
mod letterbox;
mod main_menu;
mod pixel;
mod plugins;
mod progression;
mod stage;
mod systems;
mod transitions;

#[macro_use]
extern crate lazy_static;

use crate::globals::{DEFAULT_MASTER_VOLUME, DEFAULT_MUSIC_VOLUME, DEFAULT_SFX_VOLUME};
use bevy::prelude::*;
use bevy_framepace::*;
use bevy_utils::despawn_entities;
use components::DespawnMark;
use cutscene::{data::CutsceneLayer, CutscenePlugin};
use debug::DebugPlugin;
use game::GamePlugin;
use globals::{DEFAULT_CROSSHAIR_INDEX, SCREEN_RESOLUTION, VIEWPORT_RESOLUTION};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin},
    Actionlike,
};
use letterbox::LetterboxPlugin;
use pixel::{systems::update_rectangle_position, PixelPlugin};
use plugins::movement::linear::components::{
    extra::LinearMovement2DReachCheck, TargetingPositionX, TargetingPositionY, TargetingPositionZ,
};
use seldom_pixel::prelude::*;
use stage::{
    components::placement::Depth, player::crosshair::CrosshairSettings, resources::StageTime,
    StagePlugin,
};
use systems::{
    audio::VolumeSettings,
    camera::move_camera,
    movement::{update_position_x, update_position_y},
    setup::{init_gb_input, set_framespace, spawn_camera},
    *,
};
// use transitions::spiral::TransitionVenetianPlugin;

fn main() {
    let title: String = "CARCINISATION".to_string();
    let focused: bool = false;

    let mut app = App::new();
    #[cfg(debug_assertions)]
    {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title,
                    focused,
                    resizable: true,
                    resolution: VIEWPORT_RESOLUTION.into(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            bevy_editor_pls::EditorPlugin::new(),
            bevy::diagnostic::LogDiagnosticsPlugin::default(),
            DebugPlugin,
        ));
        register_types(&mut app);
    }
    #[cfg(not(debug_assertions))]
    {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title,
                focused,
                resizable: false,
                resolution: VIEWPORT_RESOLUTION.into(),
                ..Default::default()
            }),
            ..Default::default()
        }));
    }
    app
        // TEMP
        // .insert_resource(GlobalVolume::new(0.3))
        .insert_resource(VolumeSettings(
            DEFAULT_MASTER_VOLUME,
            DEFAULT_MUSIC_VOLUME,
            DEFAULT_SFX_VOLUME,
        ))
        // Input
        .add_plugins(InputManagerPlugin::<GBInput>::default())
        .init_resource::<ActionState<GBInput>>()
        //  Setup
        .add_plugins(FramepacePlugin)
        .add_plugins(PixelPlugin::<Layer>::default())
        .add_systems(Startup, (spawn_camera, set_framespace, init_gb_input))
        // Graphics and Game
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(CrosshairSettings(DEFAULT_CROSSHAIR_INDEX))
        .add_plugins(PxPlugin::<Layer>::new(
            SCREEN_RESOLUTION,
            "palette/base.png".into(),
        ))
        // .add_plugins(TransitionVenetianPlugin)
        .add_plugins(CutscenePlugin)
        .add_plugins(LetterboxPlugin)
        // .add_plugins(MainMenuPlugin)
        .add_plugins(StagePlugin)
        .add_plugins(GamePlugin)
        .add_systems(PostStartup, trigger_game_startup)
        .add_systems(
            Update,
            (
                move_camera,
                update_position_x,
                update_position_y,
                // transition_to_game_state,
                // transition_to_main_menu_state,
                // input_exit_game,
            ),
        )
        // Cleanup
        .add_systems(PostUpdate, despawn_entities::<DespawnMark>)
        .run();
}

// TODO move to its own module
fn register_types(app: &mut App) {
    app.register_type::<Depth>()
        .register_type::<StageTime>()
        .register_type::<TargetingPositionX>()
        .register_type::<TargetingPositionY>()
        .register_type::<TargetingPositionZ>();
}

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum GBInput {
    A,
    B,
    Up,
    Down,
    Left,
    Right,
    Start,
    Select,
    // DEBUG
    DUp,
    DDown,
    DLeft,
    DRight,
    DToGame,
    DToMainMenu,
    DExit,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MidDepth {
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Zero,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum PreBackgroundDepth {
    Nine,
    Eight,
    Seven,
}

#[px_layer]
pub enum Layer {
    Skybox,

    PreBackgroundDepth(PreBackgroundDepth),
    Background,
    MidDepth(MidDepth),

    Attack,
    #[default]
    Front,
    HudBackground,
    Hud,
    Pickups,
    UIBackground,
    UI,
    CutsceneLayer(CutsceneLayer),
    // CutsceneBackground,
    // Cutscene(u8),
    // Letterbox,
    // CutsceneText,
    Transition,
}

// impl Layer {
//     pub fn get_from_depth(entity_type: StageEntityType, depth: DepthBase) -> Self {}
// }
