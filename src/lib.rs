use bevy::prelude::*;

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
