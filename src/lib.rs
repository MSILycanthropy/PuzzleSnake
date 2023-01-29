use bevy::prelude::*;

pub mod game;
pub mod level;
pub mod menu;

pub const SCALE: f32 = 16.0;

#[derive(Component)]
pub struct Player {
    pub direction: PlayerDirection,
    pub movement_timer: Timer,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
}

impl PlayerDirection {
    fn to_vec(&self) -> Vec3 {
        match self {
            Self::Up => Vec3::new(0.0, 1.0, 0.0),
            Self::Down => Vec3::new(0.0, -1.0, 0.0),
            Self::Left => Vec3::new(-1.0, 0.0, 0.0),
            Self::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Menu,
    Playing,
    GameOver,
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
