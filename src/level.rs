use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn, GameState, TextureAssets};

pub const LEVEL_SIZE: i32 = 13;

#[derive(Component)]
struct Level;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Playing, level_setup_system)
            .add_exit_system(GameState::Playing, despawn::<Level>);
    }
}

fn level_setup_system(mut commands: Commands, assets: Res<TextureAssets>) {
    for i in -LEVEL_SIZE..=LEVEL_SIZE {
        for j in -LEVEL_SIZE..=LEVEL_SIZE {
            let texture = if (i + j) % 2 == 0 {
                assets.tile_light.clone()
            } else {
                assets.tile_light.clone()
            };

            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(i as f32, j as f32, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                },
                Level,
            ));
        }
    }

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.wall_sheet.clone(),
            transform: Transform::from_xyz(-LEVEL_SIZE as f32 - 1.0, LEVEL_SIZE as f32 + 1.0, 1.0),
            sprite: TextureAtlasSprite {
                index: 0,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Level,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.wall_sheet.clone(),
            transform: Transform::from_xyz(LEVEL_SIZE as f32 + 1.0, LEVEL_SIZE as f32 + 1.0, 1.0),
            sprite: TextureAtlasSprite {
                index: 2,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Level,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.wall_sheet.clone(),
            transform: Transform::from_xyz(-LEVEL_SIZE as f32 - 1.0, -LEVEL_SIZE as f32 - 1.0, 1.0),
            sprite: TextureAtlasSprite {
                index: 4,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Level,
    ));
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.wall_sheet.clone(),
            transform: Transform::from_xyz(LEVEL_SIZE as f32 + 1.0, -LEVEL_SIZE as f32 - 1.0, 1.0),
            sprite: TextureAtlasSprite {
                index: 5,
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
        Level,
    ));

    for i in 0..4 {
        for j in -LEVEL_SIZE..=LEVEL_SIZE {
            let index = match i {
                0 | 2 => 1,
                1 | 3 => 3,
                _ => unreachable!(),
            };

            let x = match i {
                0 | 2 => j as f32,
                1 => LEVEL_SIZE as f32 + 1.0,
                3 => -LEVEL_SIZE as f32 - 1.0,
                _ => unreachable!(),
            };

            let y = match i {
                0 => LEVEL_SIZE as f32 + 1.0,
                1 | 3 => j as f32,
                2 => -LEVEL_SIZE as f32 - 1.0,
                _ => unreachable!(),
            };

            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: assets.wall_sheet.clone(),
                    transform: Transform::from_xyz(x, y, 1.0),
                    sprite: TextureAtlasSprite {
                        index,
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                },
                Level,
            ));
        }
    }
}
