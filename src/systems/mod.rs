pub mod camera;
pub mod audio;

use bevy::{app::AppExit, prelude::*, audio::Volume};
use bevy_framepace::Limiter;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};
use seldom_pixel::prelude::PxSubPosition;

use crate::{events::*, AppState, GBInput, audio::AudioSystemType};

use self::{camera::CameraPos, audio::VolumeSettings};

pub fn input_exit_game(
    gb_input_query: Query<&ActionState<GBInput>>,
    mut exit: ResMut<Events<AppExit>>,
) {
    let gb_input = gb_input_query.single();
    if gb_input.just_pressed(GBInput::DExit) {
        exit.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for game_over in game_over_event_reader.iter() {
        println!("Your final score: {}", game_over.score);
    }
}

pub fn set_framespace(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(59.727500569606);
}

pub fn spawn_camera(mut commands: Commands) {
    // let window: &Window = window_query.get_single().unwrap();
    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 10.0),
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());

    commands.spawn((PxSubPosition::default(), CameraPos));
}

pub fn spawn_gb_input(mut commands: Commands) {
    commands.spawn(InputManagerBundle::<GBInput> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::Left, GBInput::Left),
            (KeyCode::Up, GBInput::Up),
            (KeyCode::Right, GBInput::Right),
            (KeyCode::Down, GBInput::Down),
            (KeyCode::Z, GBInput::B),
            (KeyCode::X, GBInput::A),
            (KeyCode::Return, GBInput::Start),
            (KeyCode::ShiftRight, GBInput::Select),
            (KeyCode::I, GBInput::DToGame),
            (KeyCode::Back, GBInput::DToMainMenu),
            (KeyCode::Escape, GBInput::DExit),
            (KeyCode::A, GBInput::DLeft),
            (KeyCode::W, GBInput::DUp),
            (KeyCode::D, GBInput::DRight),
            (KeyCode::S, GBInput::DDown)
        ])
    });
}

pub fn transition_to_game_state(
    gb_input_query: Query<&ActionState<GBInput>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let gb_input = gb_input_query.single();
    if gb_input.just_pressed(GBInput::DToGame) {
        if app_state.get().to_owned() != AppState::Game {
            next_state.set(AppState::Game);
            println!("Entered AppState::Game");
        }
    }
}

pub fn transition_to_main_menu_state(
    gb_input_query: Query<&ActionState<GBInput>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let gb_input = gb_input_query.single();
    if gb_input.just_pressed(GBInput::DToMainMenu) {
        if app_state.get().to_owned() != AppState::MainMenu {
            // commands.insert_resource(NextState(Some(AppState::MainMenu)));
            next_state.set(AppState::MainMenu);
            println!("Entered AppState::MainMenu");
        }
    }
}

pub fn update_master_volume(
    volume_settings: Res<VolumeSettings>
) {
    let master_volume = volume_settings.0;
    GlobalVolume::new(master_volume);
}

pub fn update_music_volume(
    mut source_settings: Query<(&mut PlaybackSettings, &AudioSystemType)>,
    volume_settings: Res<VolumeSettings>
) {
    let music_volume = volume_settings.1;
    for (mut music_source_settings, audio_system_type) in source_settings.iter_mut() {
        if matches!(audio_system_type, AudioSystemType::SFX) {
            music_source_settings.volume = Volume::new_relative(music_volume);
        }
    }
}

pub fn update_sfx_volume(
    mut source_settings: Query<&mut PlaybackSettings>,
    volume_settings: Res<VolumeSettings>
) {
    let sfx_volume = volume_settings.2;
    for mut sfx_source_settings in &mut source_settings{
        sfx_source_settings.volume = Volume::new_relative(sfx_volume);
    }
}