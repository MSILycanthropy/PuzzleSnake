use std::{collections::VecDeque, time::Duration};

use bevy::{ecs::system::Command, prelude::*};
use bevy_kira_audio::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    despawn,
    enemy::{Enemy, EnemyAttack},
    music::Gameplay,
    AudioAssets, DestroyAfter, GameState, Position, TextureAssets,
};

const SNAKE_TIMESTEP: u64 = 125;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Snake>()
            .init_resource::<Direction>()
            .add_enter_system(GameState::Playing, reset_snake_system)
            .add_fixed_timestep(Duration::from_millis(SNAKE_TIMESTEP), "snake")
            .add_fixed_timestep_system(
                "snake",
                0,
                draw_snake_system
                    .run_in_state(GameState::Playing)
                    .after("movement"),
            )
            .add_fixed_timestep_system(
                "snake",
                0,
                move_snake_system
                    .run_in_state(GameState::Playing)
                    .label("movement"),
            )
            .add_system(input_system.run_in_state(GameState::Playing))
            .add_system(growth_system.run_in_state(GameState::Playing))
            .add_fixed_timestep_system(
                "snake",
                0,
                collision_system
                    .run_in_state(GameState::Playing)
                    .after("movement"),
            )
            .add_fixed_timestep_system(
                "snake",
                0,
                damage_system
                    .run_in_state(GameState::Playing)
                    .after("movement"),
            )
            .add_exit_system(GameState::Playing, despawn::<SnakeSegment>)
            .add_system(
                (|mut commands: Commands, keyboard_input: Res<Input<KeyCode>>| {
                    if keyboard_input.just_pressed(KeyCode::Space) {
                        commands.insert_resource(NextState(GameState::Playing));
                    }
                })
                .run_in_state(GameState::GameOver),
            );
    }
}

#[derive(Resource)]
pub struct Snake {
    pub segments: VecDeque<Position>,
}

impl Default for Snake {
    fn default() -> Self {
        Self {
            segments: VecDeque::from(vec![
                Position { x: 0, y: 0 },
                Position { x: -1, y: 0 },
                Position { x: -2, y: 0 },
            ]),
        }
    }
}

impl Snake {
    fn head(&self) -> &Position {
        self.segments.front().unwrap()
    }

    fn tail(&self) -> &Position {
        self.segments.back().unwrap()
    }

    fn direction(&self) -> Direction {
        let head = self.head();
        let one_before_head = self.segments.get(1).unwrap();

        match (head.x - one_before_head.x, head.y - one_before_head.y) {
            (1, 0) => Direction::Right,
            (-1, 0) => Direction::Left,
            (0, 1) => Direction::Up,
            (0, -1) => Direction::Down,
            _ => unreachable!(),
        }
    }

    pub fn damage(&mut self, amount: usize) {
        for _ in 0..amount {
            self.segments.pop_back();
        }
    }

    pub fn is_dead(&self) -> bool {
        self.segments.len() <= 3
    }
}

struct AddSnakeSegment;

impl Command for AddSnakeSegment {
    fn write(self, world: &mut World) {
        let snake = world.get_resource::<Snake>().unwrap().clone();
        let tail = snake.segments.front().unwrap();
        let one_before_tail = snake.segments.get(1).unwrap();

        let new_tail = match (tail.x - one_before_tail.x, tail.y - one_before_tail.y) {
            (1, 0) => Position {
                x: tail.x - 1,
                y: tail.y,
            },
            (-1, 0) => Position {
                x: tail.x + 1,
                y: tail.y,
            },
            (0, 1) => Position {
                x: tail.x,
                y: tail.y - 1,
            },
            (0, -1) => Position {
                x: tail.x,
                y: tail.y + 1,
            },
            _ => unreachable!(),
        };

        let mut snake = world.get_resource_mut::<Snake>().unwrap();
        snake.segments.push_back(new_tail);
    }
}

#[derive(Resource, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::Right
    }
}

#[derive(Component)]
struct SnakeSegment;

fn reset_snake_system(mut snake: ResMut<Snake>, mut direction: ResMut<Direction>) {
    *snake = Snake::default();
    *direction = Direction::default();
}

fn draw_snake_system(
    snake: Res<Snake>,
    mut commands: Commands,
    assets: Res<TextureAssets>,
    segment_entities: Query<Entity, With<SnakeSegment>>,
) {
    for entity in segment_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    snake.segments.iter().enumerate().for_each(|(i, x)| {
        if x == snake.head() {
            draw_snake_head(&mut commands, &assets, &snake);
        } else if x == snake.tail() {
            draw_snake_tail(&mut commands, &assets, &snake);
        } else {
            draw_snake_body(&mut commands, &assets, &snake, i);
        }
    });
}

fn draw_snake_head(commands: &mut Commands, assets: &Res<TextureAssets>, snake: &Snake) {
    let head = snake.head();
    let mut transform = Transform::from_xyz(head.x as f32, head.y as f32, 2.);

    let (x, y) = (head.x - snake.segments[1].x, head.y - snake.segments[1].y);
    let rotation = match (x, y) {
        (1, 0) => -std::f32::consts::FRAC_PI_2,
        (-1, 0) => std::f32::consts::FRAC_PI_2,
        (0, 1) => 0.,
        (0, -1) => std::f32::consts::PI,
        _ => 0.,
    };

    transform.rotate(Quat::from_rotation_z(rotation));

    commands.spawn((
        SpriteBundle {
            texture: assets.head.clone(),
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        SnakeSegment,
    ));
}

fn draw_snake_tail(commands: &mut Commands, assets: &Res<TextureAssets>, snake: &Snake) {
    let tail = snake.tail();
    let mut transform = Transform::from_xyz(tail.x as f32, tail.y as f32, 2.);

    let (x, y) = (
        tail.x - snake.segments[snake.segments.len() - 2].x,
        tail.y - snake.segments[snake.segments.len() - 2].y,
    );
    let rotation = match (x, y) {
        (1, 0) => std::f32::consts::FRAC_PI_2,
        (-1, 0) => -std::f32::consts::FRAC_PI_2,
        (0, 1) => std::f32::consts::PI,
        _ => 0.,
    };

    transform.rotate(Quat::from_rotation_z(rotation));

    commands.spawn((
        SpriteBundle {
            texture: assets.tail.clone(),
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        SnakeSegment,
    ));
}

fn draw_snake_body(
    commands: &mut Commands,
    assets: &Res<TextureAssets>,
    snake: &Snake,
    current_index: usize,
) {
    let current_segment = &snake.segments[current_index];
    let previous_segment = &snake.segments[current_index - 1];
    let next_segment = &snake.segments[current_index + 1];

    let mut transform = Transform::from_xyz(current_segment.x as f32, current_segment.y as f32, 2.);

    let previous_offset = Position {
        x: previous_segment.x - current_segment.x,
        y: previous_segment.y - current_segment.y,
    };

    let next_offset = Position {
        x: next_segment.x - current_segment.x,
        y: next_segment.y - current_segment.y,
    };

    let rotation = if previous_offset.x == next_offset.x {
        0.
    } else if previous_offset.y == next_offset.y {
        std::f32::consts::FRAC_PI_2
    } else {
        if (previous_offset.x == -1 && next_offset.y == -1)
            || (previous_offset.y == -1 && next_offset.x == -1)
        {
            -std::f32::consts::FRAC_PI_2
        } else if (previous_offset.x == -1 && next_offset.y == 1)
            || (previous_offset.y == 1 && next_offset.x == -1)
        {
            std::f32::consts::PI
        } else if (previous_offset.x == 1 && next_offset.y == -1)
            || (previous_offset.y == -1 && next_offset.x == 1)
        {
            0.
        } else {
            std::f32::consts::FRAC_PI_2
        }
    };

    let texture = if previous_offset.x == next_offset.x || previous_offset.y == next_offset.y {
        assets.body.clone()
    } else {
        assets.body_corner.clone()
    };

    transform.rotate(Quat::from_rotation_z(rotation));

    commands.spawn((
        SpriteBundle {
            texture,
            transform,
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        SnakeSegment,
    ));
}

fn move_snake_system(mut snake: ResMut<Snake>, direction: Res<Direction>) {
    let mut new_head = snake.head().clone();

    match *direction {
        Direction::Up => new_head.y += 1,
        Direction::Down => new_head.y -= 1,
        Direction::Left => new_head.x -= 1,
        Direction::Right => new_head.x += 1,
    }

    snake.segments.push_front(new_head);
    snake.segments.pop_back();
}

fn input_system(
    snake: Res<Snake>,
    keyboard_input: Res<Input<KeyCode>>,
    mut direction: ResMut<Direction>,
) {
    let mut new_direction = direction.clone();
    let snake_direction = snake.direction();

    if keyboard_input.pressed(KeyCode::W) | keyboard_input.pressed(KeyCode::Up) {
        new_direction = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::S) | keyboard_input.pressed(KeyCode::Down) {
        new_direction = Direction::Down;
    }
    if keyboard_input.pressed(KeyCode::A) | keyboard_input.pressed(KeyCode::Left) {
        new_direction = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::D) | keyboard_input.pressed(KeyCode::Right) {
        new_direction = Direction::Right;
    }

    if new_direction != snake_direction.opposite() {
        *direction = new_direction;
    }
}

fn growth_system(
    mut commands: Commands,
    snake: Res<Snake>,
    enemies: Query<(Entity, &Position), With<Enemy>>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
) {
    let head = snake.head();

    for (entity, enemy) in enemies.iter() {
        if head == enemy {
            commands.entity(entity).despawn();
            commands.add(AddSnakeSegment);

            gameplay_channel
                .play(audio_assets.eat.clone())
                .with_volume(0.5);
        }
    }
}

fn collision_system(
    mut commands: Commands,
    snake: Res<Snake>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
) {
    let head = snake.head();

    let mut kill_snake = || {
        gameplay_channel.play(audio_assets.death_by_bumping.clone());
        commands.insert_resource(NextState(GameState::GameOver));
    };

    if !head.in_world() {
        kill_snake();
    }

    for segment in snake.segments.iter().skip(1) {
        if head == segment {
            kill_snake();
        }
    }
}

fn damage_system(
    mut commands: Commands,
    mut snake: ResMut<Snake>,
    enemy_attacks: Query<(Entity, &Transform), With<EnemyAttack>>,
    texture_assets: Res<TextureAssets>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
) {
    for (entity, transform) in enemy_attacks.iter() {
        let transform_to_check =
            transform.translation + (Vec3::new(0.5, 0.5, 0.) * transform.rotation.to_scaled_axis());
        let enemy_attack_position = Position::from(transform_to_check.truncate());

        if snake.segments.contains(&enemy_attack_position) {
            snake.damage(1);
            commands.entity(entity).despawn();
            gameplay_channel.play(audio_assets.hit.clone());

            commands.spawn((
                SpriteBundle {
                    texture: texture_assets.effect.clone(),
                    transform: Transform::from_xyz(
                        enemy_attack_position.x as f32,
                        enemy_attack_position.y as f32,
                        3.,
                    ),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(2., 2.)),
                        ..default()
                    },
                    ..default()
                },
                DestroyAfter(Timer::from_seconds(0.25, TimerMode::Once)),
            ));

            if snake.is_dead() {
                commands.insert_resource(NextState(GameState::GameOver));
            }
        }
    }
}
