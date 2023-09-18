use bevy::prelude::*;
use seldom_pixel::{asset::*, prelude::*};

use crate::{game::resources::*, Layer};

use super::resources::GameProgress;

pub fn make_background_bundle(
    assets_sprite: &mut PxAssets<PxSprite>,
    data_handle: &Res<StageDataHandle>,
    data: &Res<Assets<StageData>>,
    game_progress: &Res<GameProgress>,
) -> Option<(PxSpriteBundle<Layer>, PxSubPosition, Name)> {
    let handle_stage_data = data_handle.0.clone();
    if let Some(stage) = data.get(&handle_stage_data) {
        let sprite = assets_sprite.load(stage.background.clone());

        return Some((
            PxSpriteBundle::<Layer> {
                sprite,
                anchor: PxAnchor::BottomLeft,
                layer: Layer::Back,
                ..default()
            },
            PxSubPosition::from(Vec2::new(0.0, 0.0)),
            Name::new("Background"),
        ));
    } else {
        return None;
    }
}

pub fn make_skybox_bundle(
    assets_sprite: &mut PxAssets<PxSprite>,
    data_handle: &Res<StageDataHandle>,
    data: &Res<Assets<StageData>>,
    game_progress: &Res<GameProgress>,
) -> Option<(PxSpriteBundle<Layer>, PxSubPosition, Name)> {
    let handle_stage_data = data_handle.0.clone();
    if let Some(stage) = data.get(&handle_stage_data) {
        let sprite = assets_sprite.load(stage.skybox.clone());

        return Some((
            PxSpriteBundle::<Layer> {
                sprite,
                anchor: PxAnchor::BottomLeft,
                canvas: PxCanvas::Camera,
                layer: Layer::Skybox,
                ..default()
            },
            PxSubPosition::from(Vec2::new(0.0, 0.0)),
            Name::new("Skybox"),
        ));
    } else {
        return None;
    }
}