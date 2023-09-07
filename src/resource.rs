use bevy::asset::Handle;
use bevy::prelude::*;
use bevy_asset_loader::prelude::AssetCollection;

pub const WOLF_SPEED: f32 = 0.005;
pub const SHEEP_SPEED: f32 = 0.003;

pub const SHEEP_VISION: f32 = 7.5;
pub const SHEEP_HUNGRY_TIME: u128 = 2000;

pub const CHUNK_SIZE: f32 = 15.;

pub const CELL_SIZE: f32 = 0.1;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(key = "wolf")]
    pub wolf: Handle<TextureAtlas>,
    #[asset(key = "sheep")]
    pub sheep: Handle<TextureAtlas>,
    #[asset(key = "grass")]
    pub grass: Handle<TextureAtlas>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(key = "font")]
    pub font: Handle<Font>,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component, Default)]
pub struct Wolf;
#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct Sheep {
    pub hungry: u128,
}

#[derive(Component, Default)]
pub struct Grass;

#[derive(Component)]
pub struct AliveText;

#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct WolfEatCount(pub u128);

#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct HungryCount(pub u128);

#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct Speed(pub f32);

#[derive(Component, Default, Deref, DerefMut, Clone, Copy)]
pub struct MoveDirection(pub Vec2);
