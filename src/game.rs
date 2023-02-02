use bevy::prelude::*;

use crate::{
    despawn,
    snake::{
        rotate_snake_head_system, rotate_snake_tail_system, snake_death_system,
        snake_growth_system, snake_input_system, snake_movement_system, snake_position_lerp_system,
        snake_setup_system, spawn_food_system, swap_snake_sprites_system, SnakeSegment,
    },
    GameState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(snake_setup_system))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(snake_input_system.before(snake_movement_system))
                    .with_system(snake_movement_system)
                    .with_system(spawn_food_system)
                    .with_system(snake_growth_system.after(snake_movement_system))
                    .with_system(snake_death_system.after(snake_growth_system))
                    .with_system(snake_position_lerp_system.after(snake_movement_system))
                    .with_system(rotate_snake_head_system.after(swap_snake_sprites_system))
                    .with_system(swap_snake_sprites_system.after(snake_movement_system))
                    .with_system(rotate_snake_tail_system.after(swap_snake_sprites_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(despawn::<SnakeSegment>),
            );
    }
}
