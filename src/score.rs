use std::cmp;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{despawn, enemy::MaxEnemies, snake::Snake, GameState};

#[derive(Resource, Default, Deref, DerefMut)]
struct Score(i32);

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct PlayButton;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_enter_system(GameState::Playing, spawn_score)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(update_score)
                    .with_system(scale_difficulty)
                    .into(),
            )
            .add_exit_system(GameState::Playing, despawn::<ScoreDisplay>)
            .add_enter_system(GameState::GameOver, spawn_game_over)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::GameOver)
                    .with_system(button_play.run_if(button_interacted::<PlayButton>))
                    .into(),
            )
            .add_exit_system(GameState::GameOver, despawn::<ScoreDisplay>);
    }
}

fn spawn_score(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    padding: UiRect {
                        top: Val::Percent(2.),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            },
            ScoreDisplay,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/impact.ttf"),
                        font_size: 80.,
                        color: Color::BLACK,
                    },
                ),
                ScoreText,
            ));
        });
}

fn update_score(
    snake: Res<Snake>,
    mut score: ResMut<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in query.iter_mut() {
        let new_score = snake.segments.len() as i32 - 3;
        score.0 = new_score;
        text.sections[0].value = format!("{}", new_score);
    }
}

fn scale_difficulty(score: Res<Score>, mut max_enemies: ResMut<MaxEnemies>) {
    let calculated = (2. * (score.0 as f32 / 5.).log2()).floor() as usize;

    max_enemies.0 = cmp::max(1, calculated);
}

fn spawn_game_over(score: Res<Score>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            ScoreDisplay,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    format!("Game Over - Score: {}", score.0),
                    TextStyle {
                        font: asset_server.load("fonts/impact.ttf"),
                        font_size: 80.,
                        color: Color::BLACK,
                    },
                ),
                ScoreText,
            ));

            parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.), Val::Px(80.)),
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(10.),
                            bottom: Val::Px(0.),
                        },
                        ..default()
                    },
                    image: UiImage {
                        0: asset_server.load("sprites/start_button.png")
                    },
                    ..default()
                },
                PlayButton,
            ));
        });
}


fn button_interacted<T: Component>(
    query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<T>)>,
) -> bool {
    query
        .iter()
        .any(|interaction| *interaction == Interaction::Clicked)
}

fn button_play(mut commands: Commands) {
    // commands.insert_resource(NextState(::Disabled));
    commands.insert_resource(NextState(GameState::Playing));
}