use bevy::prelude::*;
use bevy_sprite3d::{AtlasSprite3d, Sprite3dParams};
use rand::Rng;

use crate::resource::*;

pub fn sheep_breed(
    mut commands: Commands,
    image_assets: Res<TextureAssets>,
    mut sprite_params: Sprite3dParams,
    sheeps: Query<&Transform, With<Sheep>>,
) {
    let mut rng = rand::thread_rng();
    for _ in 0..sheeps.iter().count() / 2 {
        let x: f32 = rng.gen();
        let z: f32 = rng.gen();
        spawn_sheep(
            &mut commands,
            &image_assets,
            &mut sprite_params,
            (x - 0.5) * CHUNK_SIZE,
            (z - 0.5) * CHUNK_SIZE,
        );
    }
}

pub fn spawn_sheep(
    commands: &mut Commands,
    image_assets: &Res<TextureAssets>,
    sprite_params: &mut Sprite3dParams,
    x: f32,
    z: f32,
) {
    commands.spawn((
        AtlasSprite3d {
            atlas: image_assets.sheep.clone(),
            partial_alpha: true,
            unlit: true,
            index: 0,
            transform: Transform {
                translation: Vec3::new(x, 0.1, z),
                ..default()
            },
            ..default()
        }
        .bundle(sprite_params),
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Sheep::default(),
        Speed(SHEEP_SPEED),
        MoveDirection::default(),
    ));
}

pub fn spawn_grasses(
    mut commands: Commands,
    image_assets: Res<TextureAssets>,
    mut sprite_params: Sprite3dParams,
) {
    let mut rng = rand::thread_rng();
    let x: f32 = rng.gen();
    let z: f32 = rng.gen();
    spawn_grass(
        &mut commands,
        &image_assets,
        &mut sprite_params,
        (x - 0.5) * CHUNK_SIZE,
        (z - 0.5) * CHUNK_SIZE,
    );
}
pub fn spawn_grass(
    commands: &mut Commands,
    image_assets: &Res<TextureAssets>,
    sprite_params: &mut Sprite3dParams,
    x: f32,
    z: f32,
) {
    commands.spawn((
        AtlasSprite3d {
            atlas: image_assets.grass.clone(),
            partial_alpha: true,
            unlit: true,
            index: 0,
            transform: Transform {
                translation: Vec3::new(x, 0.05, z),
                ..default()
            },
            ..default()
        }
        .bundle(sprite_params),
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Grass,
    ));
}
