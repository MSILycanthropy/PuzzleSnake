use std::time::Duration;

use bevy::{prelude::*, render::camera::ScalingMode, window::close_on_esc};
use bevy_framepace::{debug::DiagnosticsPlugin, FramepacePlugin, FramepaceSettings, Limiter};
use super_snake::{game::GamePlugin, level::LevelPlugin, menu::MenuPlugin, GameState, SCALE};

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                window: WindowDescriptor {
                    fit_canvas_to_parent: true,
                    mode: WindowMode::Fullscreen,
                    ..default()
                },
                ..default()
            }),
    )
    .add_plugin(FramepacePlugin)
    .add_plugin(DiagnosticsPlugin)
    .add_plugin(MenuPlugin)
    .add_plugin(GamePlugin)
    .add_plugin(LevelPlugin);

    app.add_startup_system(setup_system)
        .add_system(close_on_esc);

    app.add_state(GameState::Menu);

    app.run()
}

fn setup_system(mut commands: Commands, mut settings: ResMut<FramepaceSettings>) {
    settings.limiter = Limiter::Manual(Duration::from_secs_f64(0.125));

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Auto {
        min_height: SCALE * 2.0,
        min_width: SCALE * 2.0,
    };

    commands.spawn(camera);
}
