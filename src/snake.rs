use bevy::prelude::*;
use rand::Rng;

use crate::{Frames, GameState, Position};

const FRAMES_PER_UPDATE: Frames = Frames(5);

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct MovementTimer {
    timer: Timer,
}

impl Default for MovementTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(FRAMES_PER_UPDATE.into(), TimerMode::Repeating),
        }
    }
}

#[derive(Component, Default)]
pub struct SnakeHead {
    direction: SnakeDirection,
}

#[derive(Component, Default)]
pub struct SnakeTail;

#[derive(Component, Default)]
pub struct SnakeSegment;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl Default for SnakeDirection {
    fn default() -> Self {
        Self::Right
    }
}

impl SnakeDirection {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Into<Vec3> for SnakeDirection {
    fn into(self) -> Vec3 {
        match self {
            Self::Up => Vec3::new(0.0, 1.0, 0.0),
            Self::Down => Vec3::new(0.0, -1.0, 0.0),
            Self::Left => Vec3::new(-1.0, 0.0, 0.0),
            Self::Right => Vec3::new(1.0, 0.0, 0.0),
        }
    }
}

#[derive(Resource)]
pub struct LastTailPosition(pub Option<Position>);

#[derive(Component)]
pub struct Food;

pub fn snake_setup_system(mut commands: Commands) {
    // let head_sprite = asset_server.load("sprites/head.png");
    // let body_sprite = asset_server.load("sprites/body.png");
    // let tail_sprite = asset_server.load("sprites/tail.png");

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(0.75, 0.75)),
                ..default()
            },
            ..default()
        },
        SnakeHead::default(),
        SnakeSegment,
        Position { x: 0, y: 0 },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-1.0, 0.0, 1.0),
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            ..default()
        },
        SnakeSegment,
        Position { x: -1, y: 0 },
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-2.0, 0.0, 1.0),
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(0.5, 0.5)),
                ..default()
            },
            ..default()
        },
        SnakeTail::default(),
        SnakeSegment,
        Position { x: -2, y: 0 },
    ));

    commands.insert_resource(LastTailPosition(None));
    commands.insert_resource(MovementTimer::default());
}

pub fn snake_input_system(
    mut head_query: Query<&mut SnakeHead>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut head = head_query.single_mut();

    let dir = if keyboard_input.just_pressed(KeyCode::W) {
        SnakeDirection::Up
    } else if keyboard_input.just_pressed(KeyCode::S) {
        SnakeDirection::Down
    } else if keyboard_input.just_pressed(KeyCode::A) {
        SnakeDirection::Left
    } else if keyboard_input.just_pressed(KeyCode::D) {
        SnakeDirection::Right
    } else {
        head.direction
    };

    if dir != head.direction.opposite() {
        head.direction = dir;
    }
}

pub fn snake_movement_system(
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut head_query: Query<(&mut Position, &SnakeHead)>,
    mut segments_query: Query<&mut Position, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    if !timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    let (mut head_position, head) = head_query.single_mut();

    let mut last_position = *head_position;

    match &head.direction {
        SnakeDirection::Up => {
            head_position.y += 1;
        }
        SnakeDirection::Down => {
            head_position.y -= 1;
        }
        SnakeDirection::Left => {
            head_position.x -= 1;
        }
        SnakeDirection::Right => {
            head_position.x += 1;
        }
    }

    for mut segment_position in segments_query.iter_mut() {
        std::mem::swap(&mut last_position, &mut segment_position);
    }

    last_tail_position.0 = Some(last_position);
}

pub fn snake_position_lerp_system(
    mut position_query: Query<(&mut Transform, &Position), With<SnakeSegment>>,
) {
    for (mut transform, position) in position_query.iter_mut() {
        let old_translation = transform.translation;
        let new_translation = Vec3::new(position.x as f32, position.y as f32, old_translation.z);

        transform.translation = old_translation.lerp(new_translation, 0.5);
    }
}

// TODO: This might be how we want movement to be when we have actual sprite
// pub fn snake_segment_position_lerp_system(
//     mut position_query: Query<
//         (&mut Transform, &Position),
//         (With<SnakeSegment>, Without<SnakeHead>),
//     >,
// ) {
//     for (mut transform, position) in position_query.iter_mut() {
//         transform.translation = Vec3::new(
//             position.x as f32,
//             position.y as f32,
//             transform.translation.z,
//         );
//     }
// }

pub fn spawn_food_system(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let prob = rng.gen_range(0.0..1.0);

    if prob > 0.99 {
        return;
    }

    let x = rng.gen_range(-10..10);
    let y = rng.gen_range(-10..10);

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x as f32, y as f32, 1.0),
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(0.75, 0.75)),
                ..default()
            },
            ..default()
        },
        Food,
        Position { x, y },
    ));
}

pub fn snake_growth_system(
    mut commands: Commands,
    food_query: Query<(Entity, &Position), With<Food>>,
    head_query: Query<&Position, With<SnakeHead>>,
    last_tail_position: Res<LastTailPosition>,
) {
    let head_position = head_query.single();

    for (food_entity, food_position) in food_query.iter() {
        if let Some(last_tail_position) = last_tail_position.0 {
            if food_position == head_position {
                commands.entity(food_entity).despawn();

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(
                            last_tail_position.x as f32,
                            last_tail_position.y as f32,
                            1.0,
                        ),
                        sprite: Sprite {
                            color: Color::rgb(0.0, 1.0, 0.0),
                            custom_size: Some(Vec2::new(0.5, 0.5)),
                            ..default()
                        },
                        ..default()
                    },
                    SnakeTail::default(),
                    SnakeSegment,
                    last_tail_position,
                ));
            }
        }
    }
}

pub fn snake_death_system(
    segment_query: Query<&Position, (With<SnakeSegment>, Without<SnakeHead>)>,
    head_query: Query<&Position, With<SnakeHead>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let head_position = head_query.single();

    for segment_position in segment_query.iter() {
        if segment_position == head_position {
            println!("You died!");

            let _ = game_state.set(GameState::GameOver);
        }
    }
}
