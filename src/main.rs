use bevy::{prelude::*, render::camera::ScalingMode, window::close_on_esc};
use bevy_asset_loader::prelude::*;
use bevy_framepace::{debug::DiagnosticsPlugin, FramepacePlugin, FramepaceSettings, Limiter};
use bevy_kira_audio::prelude::*;
use super_snake::{
    enemy::EnemyPlugin, game::GamePlugin, level::LevelPlugin, menu::MenuPlugin, music::MusicPlugin,
    AudioAssets, GameState, TextureAssets, SCALE,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    fit_canvas_to_parent: true,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                },
                ..default()
            }),
    )
    .add_plugin(FramepacePlugin)
    .add_plugin(DiagnosticsPlugin)
    .add_plugin(AudioPlugin)
    .add_plugin(MenuPlugin)
    .add_plugin(GamePlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(MusicPlugin)
    .add_plugin(LevelPlugin);

    app.add_startup_system(setup_system)
        .add_system(close_on_esc);

    app.add_loading_state(
        LoadingState::new(GameState::AssetsLoading)
            .continue_to_state(GameState::Menu)
            .with_collection::<TextureAssets>()
            .with_collection::<AudioAssets>(),
    );

    app.add_state(GameState::AssetsLoading);

    app.run()
}

fn setup_system(mut commands: Commands, mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(60.0);

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Auto {
        min_height: SCALE * 2.0,
        min_width: SCALE * 2.0,
    };

    commands.spawn(camera);
}
