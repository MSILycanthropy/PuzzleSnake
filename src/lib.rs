use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use level::LEVEL_SIZE;

pub mod enemy;
pub mod game;
pub mod level;
pub mod menu;
pub mod music;
pub mod snake;

pub const SCALE: i32 = 32;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    AssetsLoading,
    Menu,
    Playing,
    GameOver,
}

#[derive(Component, Clone, Copy, Eq, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn in_world(&self) -> bool {
        self.x >= -LEVEL_SIZE
            && self.x <= LEVEL_SIZE
            && self.y >= -LEVEL_SIZE
            && self.y <= LEVEL_SIZE
    }
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        Self {
            x: vec.x.floor() as i32,
            y: vec.y.floor() as i32,
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "sprites/tile_dark.png")]
    pub tile_dark: Handle<Image>,
    #[asset(path = "sprites/tile_light.png")]
    pub tile_light: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 2))]
    #[asset(path = "sprites/wall_sheet.png")]
    pub wall_sheet: Handle<TextureAtlas>,

    #[asset(path = "sprites/head.png")]
    pub head: Handle<Image>,
    #[asset(path = "sprites/body.png")]
    pub body: Handle<Image>,
    #[asset(path = "sprites/body_corner.png")]
    pub body_corner: Handle<Image>,
    #[asset(path = "sprites/tail.png")]
    pub tail: Handle<Image>,
    #[asset(path = "sprites/effect.png")]
    pub effect: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/wizard_sheet.png")]
    pub wizard_sheet: Handle<TextureAtlas>,
    #[asset(path = "sprites/projectile.png")]
    pub projectile: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "music/gameplay.wav")]
    pub gameplay_music: Handle<AudioSource>,
    #[asset(path = "music/menu.wav")]
    pub menu_music: Handle<AudioSource>,

    #[asset(path = "sounds/wizard_prepare.ogg")]
    pub wizard_prepare: Handle<AudioSource>,
    #[asset(path = "sounds/wizard_attack.ogg")]
    pub wizard_attack: Handle<AudioSource>,

    #[asset(path = "sounds/hit.ogg")]
    pub hit: Handle<AudioSource>,
    #[asset(path = "sounds/death_by_bumping.ogg")]
    pub death_by_bumping: Handle<AudioSource>,
    #[asset(path = "sounds/eat.ogg")]
    pub eat: Handle<AudioSource>,
}

pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// TODO: Anyhow for errors
pub fn error_handler(In(result): In<Result<(), String>>) {
    if let Err(error) = result {
        error!("Error: {}", error);
    }
}
