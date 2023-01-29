use bevy::prelude::*;

use crate::{GameState, Player, PlayerDirection};

const PLAYER_SPRITE: &str = "sprites/player.png";

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(player_setup_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(player_movement_system),
        );
    }
}

fn player_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_sprite = asset_server.load(PLAYER_SPRITE);

    commands.spawn((
        Player {
            direction: PlayerDirection::Right,
            movement_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        },
        SpriteBundle {
            texture: player_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..default()
            },
            ..default()
        },
    ));
}

fn player_movement_system(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut player) = player_query.single_mut();

    player.movement_timer.tick(time.delta());

    if keyboard_input.just_pressed(KeyCode::W) {
        player.direction = PlayerDirection::Up;
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        player.direction = PlayerDirection::Down;
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        player.direction = PlayerDirection::Left;
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        player.direction = PlayerDirection::Right;
    }

    if player.movement_timer.just_finished() {
        transform.translation += player.direction.to_vec();
    }
}
