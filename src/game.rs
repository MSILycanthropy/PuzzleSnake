use bevy::prelude::*;

use crate::{
    enemy::EnemyPlugin, level::LevelPlugin, menu::MenuPlugin, music::MusicPlugin,
    score::ScorePlugin, snake::SnakePlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LevelPlugin)
            .add_plugin(SnakePlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(MusicPlugin)
            .add_plugin(ScorePlugin);
    }
}
