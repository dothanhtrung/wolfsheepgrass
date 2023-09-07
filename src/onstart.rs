use bevy::core_pipeline::bloom::BloomSettings;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::prelude::*;
use bevy_panorbit_camera::PanOrbitCamera;
use bevy_sprite3d::{AtlasSprite3d, AtlasSprite3dComponent, Sprite3dParams};
use rand::Rng;

use crate::resource::*;
use crate::spawn::{spawn_grass, spawn_sheep};
use crate::GameState;

pub fn setup(
    mut commands: Commands,
    image_assets: Res<TextureAssets>,
    ui_assets: Res<UiAssets>,
    mut sprite_params: Sprite3dParams,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 10.0, 2.0),
        ..default()
    });
    // camera
    commands.spawn((
        PanOrbitCamera::default(),
        Camera3dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 13., 14.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(),
    ));

    // cell
    commands.spawn(PbrBundle {
        mesh: sprite_params
            .meshes
            .add(shape::Plane::from_size(CHUNK_SIZE + 0.5).into()),
        material: sprite_params
            .materials
            .add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Alive sheeps: ",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 45.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        AliveText,
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                ", dead by Wolf: ",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 45.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        WolfEatCount::default(),
    ));
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                ", dead by Hungry: ",
                TextStyle {
                    font: ui_assets.font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 45.0,
                color: Color::GOLD,
                ..default()
            }),
        ]),
        HungryCount::default(),
    ));

    commands.spawn((
        AtlasSprite3d {
            atlas: image_assets.wolf.clone(),
            partial_alpha: true,
            unlit: true,
            index: 0,
            transform: Transform {
                translation: Vec3::new(0., 0.15, 0.),
                ..default()
            },
            ..default()
        }
        .bundle(&mut sprite_params),
        AnimationTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
        Wolf,
        Speed(WOLF_SPEED),
        MoveDirection::default(),
    ));

    let mut rng = rand::thread_rng();
    for _ in 0..10 {
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

    for _ in 0..50 {
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

    next_state.set(GameState::InGame);
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut AtlasSprite3dComponent)>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % sprite.atlas.len();
        }
    }
}
