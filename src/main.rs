use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::window::WindowResolution;
use bevy_asset_loader::prelude::*;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_sprite3d::Sprite3dPlugin;

use crate::ingame::*;
use crate::onstart::*;
use crate::resource::*;
use crate::spawn::*;

mod ingame;
mod onstart;
mod resource;
mod spawn;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    InMenu,
    InGame,
}

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Wolf Sheep Grass"),
                    resolution: WindowResolution::new(1920., 1080.),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(PanOrbitCameraPlugin)
    .add_plugins(Sprite3dPlugin)
    .add_state::<GameState>()
    .add_loading_state(LoadingState::new(GameState::Loading).continue_to_state(GameState::InMenu))
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        GameState::Loading,
        "texture.assets.ron",
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        GameState::Loading,
        "ui.assets.ron",
    )
    .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, UiAssets>(GameState::Loading)
    .add_systems(OnEnter(GameState::InMenu), setup)
    .add_systems(
        Update,
        (animate_sprite, moving, hungry, eat).run_if(in_state(GameState::InGame)),
    )
    .add_systems(
        Update,
        change_direction
            .run_if(in_state(GameState::InGame))
            .run_if(on_timer(Duration::from_secs_f32(0.1))),
    )
    .add_systems(
        Update,
        sheep_breed.run_if(on_timer(Duration::from_secs_f32(1.2))),
    )
    .add_systems(
        Update,
        spawn_grasses.run_if(on_timer(Duration::from_secs_f32(0.05))),
    )
    .run();
}
