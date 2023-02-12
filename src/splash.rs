use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn, GameState};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_splash_system)
            .init_resource::<SplashTimer>()
            .add_system(splash_delay_system.run_in_state(GameState::SplashScreen))
            .add_exit_system(GameState::SplashScreen, despawn::<OnSplash>);
    }
}

#[derive(Component)]
struct OnSplash;

#[derive(Resource)]
struct SplashTimer {
    timer: Timer,
}

impl Default for SplashTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

fn setup_splash_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in -40..41 {
        for j in -22..24 {
            commands.spawn((SpriteBundle {
                texture: asset_server.load("ui/tile_dark.png"),
                transform: Transform {
                    translation: Vec3::new(i as f32, j as f32, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    ..default()
                },
                ..default()
            },));
        }
    }

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            OnSplash,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(640.0), Val::Px(200.0)),
                    ..default()
                },
                image: asset_server.load("ui/logo.png").into(),
                ..default()
            });
        });
}

fn splash_delay_system(
    time: Res<Time>,
    mut commands: Commands,
    mut splash_timer: ResMut<SplashTimer>,
) {
    if !splash_timer.timer.tick(time.delta()).finished() {
        return;
    }

    commands.insert_resource(NextState(GameState::Menu));
}
