use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod game;
pub mod level;
pub mod menu;
pub mod snake;

pub const SCALE: f32 = 16.0;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

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
}

impl std::ops::Deref for Position {
    type Target = Position;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "sprites/head.png")]
    pub head: Handle<Image>,
    #[asset(path = "sprites/body.png")]
    pub body: Handle<Image>,
    #[asset(path = "sprites/body_corner.png")]
    pub body_corner: Handle<Image>,
    #[asset(path = "sprites/tail.png")]
    pub tail: Handle<Image>,
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 3, rows = 1))]
    #[asset(path = "sprites/wizard_sheet.png")]
    pub wizard_sheet: Handle<TextureAtlas>,
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
