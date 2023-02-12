use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_kira_audio::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};
use iyes_loopless::prelude::*;
use super_snake::{
    despawn_after, game::GamePlugin, AudioAssets, GameState, TextureAssets, UiAssets, SCALE,
};

fn main() {
    let mut app = App::new();

    app.add_loopless_state(GameState::AssetsLoading)
        .add_loading_state(
            LoadingState::new(GameState::AssetsLoading)
                .continue_to_state(GameState::Menu)
                .with_collection::<TextureAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<UiAssets>(),
        )
        .add_startup_system(setup_system)
        .add_system(despawn_after)
        .add_plugins(
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
        .add_plugin(AudioPlugin)
        .add_plugin(PixelCameraPlugin)
        .add_plugin(GamePlugin);

    app.run()
}

fn setup_system(mut commands: Commands, mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(60.);

    let camera = PixelCameraBundle::from_zoom(SCALE);

    commands.spawn(camera);
}
