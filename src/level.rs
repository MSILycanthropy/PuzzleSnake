use bevy::prelude::*;

use crate::{despawn, GameState};

pub const LEVEL_SIZE: i32 = 13;

#[derive(Component)]
struct Level;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(level_setup_system))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawn::<Level>));
    }
}

fn level_setup_system(mut commands: Commands) {
    let gray = Color::rgb(0.5, 0.5, 0.5);
    let black = Color::rgb(0.2, 0.2, 0.2);

    for i in -LEVEL_SIZE..=LEVEL_SIZE {
        for j in -LEVEL_SIZE..=LEVEL_SIZE {
            let color = if (i + j) % 2 == 0 { gray } else { black };

            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(i as f32, j as f32, 0.0),
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        color,
                        ..default()
                    },
                    ..default()
                },
                Level,
            ));
        }
    }
}
