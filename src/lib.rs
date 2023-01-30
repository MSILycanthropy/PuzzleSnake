use bevy::prelude::*;

pub mod game;
pub mod level;
pub mod menu;
pub mod snake;

pub const SCALE: f32 = 16.0;
pub const FPS: f64 = 60.0;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
}

#[derive(Component, Clone, Copy, Eq, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub struct Frames(pub u32);

impl Into<f32> for Frames {
    fn into(self) -> f32 {
        (self.0 as f32) / (FPS as f32)
    }
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
