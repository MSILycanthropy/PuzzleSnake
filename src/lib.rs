use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use level::LEVEL_SIZE;

pub mod enemy;
pub mod game;
pub mod level;
pub mod menu;
pub mod music;
pub mod score;
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
        self.x >= -LEVEL_SIZE.x
            && self.x <= LEVEL_SIZE.x
            && self.y >= -LEVEL_SIZE.y
            && self.y <= LEVEL_SIZE.y
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

#[derive(Component, Deref, DerefMut)]
pub struct DestroyAfter(Timer);

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
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

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "ui/exit_button.png")]
    pub exit_button: Handle<Image>,
    #[asset(path = "ui/start_button.png")]
    pub start_button: Handle<Image>,
    #[asset(path = "ui/game_over.png")]
    pub game_over: Handle<Image>,
    #[asset(path = "ui/tile_dark.png")]
    pub tile_dark: Handle<Image>,

    #[asset(path = "ui/0.png")]
    pub zero: Handle<Image>,
    #[asset(path = "ui/1.png")]
    pub one: Handle<Image>,
    #[asset(path = "ui/2.png")]
    pub two: Handle<Image>,
    #[asset(path = "ui/3.png")]
    pub three: Handle<Image>,
    #[asset(path = "ui/4.png")]
    pub four: Handle<Image>,
    #[asset(path = "ui/5.png")]
    pub five: Handle<Image>,
    #[asset(path = "ui/6.png")]
    pub six: Handle<Image>,
    #[asset(path = "ui/7.png")]
    pub seven: Handle<Image>,
    #[asset(path = "ui/8.png")]
    pub eight: Handle<Image>,
    #[asset(path = "ui/9.png")]
    pub nine: Handle<Image>,
}

pub fn despawn<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in to_despawn.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn despawn_after(
    mut commands: Commands,
    mut query: Query<(Entity, &mut DestroyAfter)>,
    time: Res<Time>,
) {
    for (entity, mut destroy_after) in query.iter_mut() {
        if destroy_after.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// TODO: Anyhow for errors
pub fn error_handler(In(result): In<Result<(), String>>) {
    if let Err(error) = result {
        error!("Error: {}", error);
    }
}
