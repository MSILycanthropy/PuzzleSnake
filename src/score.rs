use std::cmp;

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    despawn,
    enemy::MaxEnemies,
    menu::{button_exit, button_interacted, button_play, ExitButton, PlayButton},
    snake::Snake,
    GameState, UiAssets,
};

#[derive(Resource, Default, Deref, DerefMut)]
struct Score(i32);

#[derive(Component)]
struct ScoreDisplay;

#[derive(Component)]
struct ScoreText;

#[derive(Bundle)]
struct ScoreBundle {
    image_bundle: ImageBundle,
    score_text: ScoreText,
}

impl ScoreBundle {
    fn new(char: &char, ui_assets: &UiAssets) -> Self {
        Self {
            image_bundle: ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(96.), Val::Px(96.)),
                    ..default()
                },
                image: match char {
                    '0' => ui_assets.zero.clone(),
                    '1' => ui_assets.one.clone(),
                    '2' => ui_assets.two.clone(),
                    '3' => ui_assets.three.clone(),
                    '4' => ui_assets.four.clone(),
                    '5' => ui_assets.five.clone(),
                    '6' => ui_assets.six.clone(),
                    '7' => ui_assets.seven.clone(),
                    '8' => ui_assets.eight.clone(),
                    '9' => ui_assets.nine.clone(),
                    _ => unreachable!(),
                }
                .into(),
                ..default()
            },
            score_text: ScoreText,
        }
    }
}

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
                    .with_system(button_exit.run_if(button_interacted::<ExitButton>))
                    .into(),
            )
            .add_exit_system(GameState::GameOver, despawn::<ScoreDisplay>);
    }
}

fn spawn_score(mut commands: Commands, ui_assets: Res<UiAssets>) {
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
                ImageBundle {
                    style: Style {
                        size: Size::new(Val::Px(64.), Val::Px(64.)),
                        ..default()
                    },
                    image: ui_assets.zero.clone().into(),
                    ..default()
                },
                ScoreText,
            ));
        });
}

fn update_score(
    snake: Res<Snake>,
    score_display: Query<Entity, With<ScoreDisplay>>,
    query: Query<Entity, With<ScoreText>>,
    ui_assets: Res<UiAssets>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    score.0 = snake.segments.len() as i32 - 3;

    if score.0 < 0 {
        score.0 = 0;
    }

    let score_display_entity = score_display.single();

    let score_entities = score
        .0
        .to_string()
        .chars()
        .map(|char| commands.spawn(ScoreBundle::new(&char, &ui_assets)).id())
        .collect::<Vec<_>>();

    commands
        .entity(score_display_entity)
        .push_children(score_entities.as_slice());
}

fn scale_difficulty(score: Res<Score>, mut max_enemies: ResMut<MaxEnemies>) {
    let calculated = (2. * (score.0 as f32 / 5.).log2()).floor() as usize;

    max_enemies.0 = cmp::max(1, calculated);
}

fn spawn_game_over(score: Res<Score>, mut commands: Commands, ui_assets: Res<UiAssets>) {
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
            parent.spawn(NodeBundle::default()).with_children(|parent| {
                score.0.to_string().chars().for_each(|char| {
                    parent.spawn(ScoreBundle::new(&char, &ui_assets));
                });
            });

            parent.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(550.), Val::Px(100.)),
                    margin: UiRect {
                        top: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                image: ui_assets.game_over.clone().into(),
                ..default()
            });

            parent.spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.), Val::Px(80.)),
                        margin: UiRect {
                            top: Val::Px(10.),
                            ..default()
                        },
                        ..default()
                    },
                    image: ui_assets.start_button.clone().into(),
                    ..default()
                },
                PlayButton,
            ));

            #[cfg(not(target_arch = "wasm32"))]
            parent.spawn((
                ButtonBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Px(200.), Val::Px(80.)),
                        margin: UiRect {
                            top: Val::Px(20.),
                            ..default()
                        },
                        ..default()
                    },
                    image: ui_assets.exit_button.clone().into(),
                    ..default()
                },
                ExitButton,
            ));
        });
}
