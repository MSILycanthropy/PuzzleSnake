use bevy::prelude::*;

use crate::{despawn, snake::*, GameState};

pub struct GamePlugin;
// TODO: This should.. be in SnakePlugin?
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(snake_setup_system))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(snake_death_system)
                    .with_system(delete_grow_effect_system)
                    .with_system(tick_snake_timers)
                    .with_system(snake_input_system.after(tick_snake_timers))
                    .with_system(snake_movement_system.after(snake_input_system))
                    .with_system(snake_growth_system.after(snake_movement_system))
                    .with_system(snake_position_lerp_system.after(snake_movement_system))
                    .with_system(snake_growth_system.after(snake_movement_system))
                    .with_system(snake_death_system.after(snake_growth_system))
                    .with_system(snake_position_lerp_system.after(snake_movement_system))
                    .with_system(rotate_snake_head_system.after(swap_snake_sprites_system))
                    .with_system(swap_snake_sprites_system.after(snake_movement_system))
                    .with_system(rotate_snake_tail_system.after(swap_snake_sprites_system)),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(despawn::<SnakeSegment>),
            )
            .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(
                |keyboard_input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>| {
                    if keyboard_input.just_pressed(KeyCode::Space) {
                        let _ = state.set(GameState::Playing);
                    }
                },
            ));
    }
}
