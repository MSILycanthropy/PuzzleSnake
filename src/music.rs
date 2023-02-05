use crate::{AudioAssets, GameState};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct Gameplay;

#[derive(Resource)]
struct Background;

#[derive(Resource)]
struct Menu;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<Menu>()
            .add_audio_channel::<Background>()
            .add_audio_channel::<Gameplay>()
            .add_system_set(
                SystemSet::on_enter(GameState::Menu).with_system(play_menu_music_system),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(stop_menu_music_system))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(play_music_system))
            .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(stop_music_system));
    }
}

// TODO: This can probably be made generic
// TODO: Volumes are kinda weird. I just chose some numbers that felt alright.
fn play_music_system(assets: Res<AudioAssets>, background: Res<AudioChannel<Background>>) {
    background
        .play(assets.gameplay_music.clone())
        .fade_in(AudioTween::new(
            Duration::from_secs_f32(1.5),
            AudioEasing::OutPowi(1),
        ))
        .with_volume(0.33)
        .looped();
}

fn stop_music_system(background: Res<AudioChannel<Background>>) {
    background.stop().fade_out(AudioTween::new(
        Duration::from_secs_f32(1.5),
        AudioEasing::OutPowi(1),
    ));
}

fn play_menu_music_system(assets: Res<AudioAssets>, menu: Res<AudioChannel<Menu>>) {
    menu.play(assets.menu_music.clone())
        .with_volume(0.75)
        .looped();
}

fn stop_menu_music_system(menu: Res<AudioChannel<Menu>>) {
    menu.stop().fade_out(AudioTween::new(
        Duration::from_secs_f32(1.5),
        AudioEasing::OutPowi(1),
    ));
}
