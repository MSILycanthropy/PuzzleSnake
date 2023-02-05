use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{enemy::Enemy, music::Gameplay, AudioAssets, GameState, Position, TextureAssets};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct SnakeHead {
    direction: SnakeDirection,
}

#[derive(Component, Default)]
pub struct SnakeTail;

#[derive(Component, Default)]
pub struct SnakeSegment;

#[derive(Resource)]
pub struct SnakeMoveTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct GrowEffect {
    pub timer: Timer,
}

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

    fn from_tuple(tuple: (i32, i32)) -> Result<Self, ()> {
        match tuple {
            (0, 1) => Ok(Self::Up),
            (0, -1) => Ok(Self::Down),
            (-1, 0) => Ok(Self::Left),
            (1, 0) => Ok(Self::Right),
            _ => Err(()),
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

pub fn snake_setup_system(mut commands: Commands, assets: Res<TextureAssets>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            texture: assets.head.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        SnakeHead::default(),
        SnakeSegment,
        Position { x: 0, y: 0 },
    ));

    for i in 1..=2 {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(-(i as f32), 0.0, 1.0),
                texture: assets.body.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            SnakeSegment,
            Position { x: -i, y: 0 },
        ));
    }

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-3.0, 0.0, 1.0),
            texture: assets.tail.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        SnakeTail::default(),
        SnakeSegment,
        Position { x: -3, y: 0 },
    ));

    commands.insert_resource(SnakeMoveTimer {
        timer: Timer::from_seconds(0.125, TimerMode::Repeating),
    });
    commands.insert_resource(LastTailPosition(None));
}

pub fn tick_snake_timers(time: Res<Time>, mut timer: ResMut<SnakeMoveTimer>) {
    timer.timer.tick(time.delta());
}

pub fn snake_input_system(
    mut head_query: Query<&mut SnakeHead>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut head = head_query.single_mut();

    let mut dir = head.direction;

    if keyboard_input.pressed(KeyCode::W) {
        dir = SnakeDirection::Up;
    }

    if keyboard_input.pressed(KeyCode::S) {
        dir = SnakeDirection::Down;
    }

    if keyboard_input.pressed(KeyCode::A) {
        dir = SnakeDirection::Left;
    }

    if keyboard_input.pressed(KeyCode::D) {
        dir = SnakeDirection::Right;
    }

    if dir != head.direction.opposite() {
        head.direction = dir;
    }
}

pub fn snake_movement_system(
    move_timer: Res<SnakeMoveTimer>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut head_query: Query<(&mut Position, &SnakeHead)>,
    mut segments_query: Query<&mut Position, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    if !move_timer.timer.just_finished() {
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
    move_timer: Res<SnakeMoveTimer>,
    mut position_query: Query<(&mut Transform, &Position), With<SnakeSegment>>,
) {
    if !move_timer.timer.just_finished() {
        return;
    }

    for (mut transform, position) in position_query.iter_mut() {
        transform.translation = Vec3::new(
            position.x as f32,
            position.y as f32,
            transform.translation.z,
        );
    }
}

pub fn rotate_snake_head_system(
    timer: Res<SnakeMoveTimer>,
    mut head_query: Query<(&mut Transform, &SnakeHead)>,
) {
    if !timer.timer.just_finished() {
        return;
    }

    let (mut transform, head) = head_query.single_mut();

    match &head.direction {
        SnakeDirection::Up => {
            transform.rotation = Quat::from_rotation_z(0.);
        }
        SnakeDirection::Down => {
            transform.rotation = Quat::from_rotation_z(std::f32::consts::PI);
        }
        SnakeDirection::Left => {
            transform.rotation = Quat::from_rotation_z(std::f32::consts::PI / 2.);
        }
        SnakeDirection::Right => {
            transform.rotation = Quat::from_rotation_z(-std::f32::consts::PI / 2.);
        }
    }
}

pub fn rotate_snake_tail_system(
    mut tail_query: Query<(&Position, &mut Transform, &SnakeTail)>,
    mut segments_query: Query<
        &Position,
        (With<SnakeSegment>, Without<SnakeHead>, Without<SnakeTail>),
    >,
) {
    let mut tail = tail_query.iter_mut().next().unwrap();

    let last_segment = if let Some(last_segment) = segments_query.iter_mut().last() {
        last_segment
    } else {
        return;
    };

    let (tail_x, tail_y) = (tail.0.x, tail.0.y);
    let (last_segment_x, last_segment_y) = (last_segment.x, last_segment.y);

    let (dx, dy) = (tail_x - last_segment_x, tail_y - last_segment_y);

    let rotation = match (dx, dy) {
        (0, 1) => Quat::from_rotation_z(std::f32::consts::PI),
        (0, -1) => Quat::from_rotation_z(0.),
        (1, 0) => Quat::from_rotation_z(std::f32::consts::PI / 2.),
        (-1, 0) => Quat::from_rotation_z(-std::f32::consts::PI / 2.),
        _ => Quat::from_rotation_z(0.),
    };

    tail.1.rotation = rotation;
}

pub fn swap_snake_sprites_system(
    assets: Res<TextureAssets>,
    mut segments_query: Query<(&Position, &mut Handle<Image>, &mut Transform), With<SnakeSegment>>,
) {
    // Oh my god this is incredibly dumb.
    let segments_vec = segments_query.iter_mut().collect::<Vec<_>>();
    let asset_rotation_map = segments_vec
        .iter()
        .map(|(pos, _, _)| *pos)
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .collect::<Vec<_>>()
        .windows(3)
        .map(|window| {
            let (_, seg1_pos) = window[0];
            let (i2, seg2_pos) = window[1];
            let (_, seg3_pos) = window[2];

            let asset = if seg3_pos.x != seg1_pos.x && seg3_pos.y != seg1_pos.y {
                assets.body_corner.clone()
            } else {
                assets.body.clone()
            };

            let slope = (seg3_pos.y - seg1_pos.y) as f32 / (seg3_pos.x - seg1_pos.x) as f32;
            let rotation = if seg3_pos.x != seg1_pos.x && seg3_pos.y != seg1_pos.y {
                let tuple = (seg2_pos.x - seg3_pos.x, seg2_pos.y - seg3_pos.y);
                let direction = SnakeDirection::from_tuple(tuple);

                if let Ok(direction) = direction {
                    snake_direction_to_rotation(direction, slope)
                } else {
                    0.
                }
            } else {
                let tuple = (seg2_pos.x - seg3_pos.x, seg2_pos.y - seg3_pos.y);
                let direction = SnakeDirection::from_tuple(tuple);

                if let Ok(direction) = direction {
                    match direction {
                        SnakeDirection::Up => 0.,
                        SnakeDirection::Down => std::f32::consts::PI,
                        SnakeDirection::Left => std::f32::consts::PI / 2.,
                        SnakeDirection::Right => -std::f32::consts::PI / 2.,
                    }
                } else {
                    0.
                }
            };

            (i2, asset, rotation)
        })
        .collect::<Vec<_>>();

    for (i, (_, mut handle, mut transform)) in segments_query.iter_mut().enumerate() {
        if let Some((_, asset, corner_rotation)) =
            asset_rotation_map.iter().find(|(i2, _, _)| i2 == &i)
        {
            *handle = asset.clone();
            transform.rotation = Quat::from_rotation_z(*corner_rotation);
        }
    }
}

fn snake_direction_to_rotation(direction: SnakeDirection, slope: f32) -> f32 {
    match direction {
        SnakeDirection::Up => {
            if slope > 0. {
                0.
            } else {
                -std::f32::consts::PI / 2.
            }
        }
        SnakeDirection::Down => {
            if slope > 0. {
                std::f32::consts::PI
            } else {
                std::f32::consts::PI / 2.
            }
        }
        SnakeDirection::Left => {
            if slope > 0. {
                0.
            } else {
                std::f32::consts::PI / 2.
            }
        }
        SnakeDirection::Right => {
            if slope > 0. {
                std::f32::consts::PI
            } else {
                -std::f32::consts::PI / 2.
            }
        }
    }
}

// TODO: Spawn the tail with the correct rotation.
pub fn snake_growth_system(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
    move_timer: Res<SnakeMoveTimer>,
    tail_query: Query<(&Transform, &Position), With<SnakeTail>>,
    enemy_query: Query<(Entity, &Position), With<Enemy>>,
    head_query: Query<&Position, With<SnakeHead>>,
    assets: Res<TextureAssets>,
) {
    if !move_timer.timer.just_finished() {
        return;
    }

    let head_position = head_query.single();

    for (enemy_entity, enemy_position) in enemy_query.iter() {
        if enemy_position == head_position {
            let (transform, position) = tail_query.single();

            // TODO: this line causes a warning in bevy for some reason now?
            commands.entity(enemy_entity).despawn();

            commands.spawn((
                SpriteBundle {
                    texture: assets.body.clone(),
                    transform: transform.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1., 1.)),
                        ..default()
                    },
                    ..default()
                },
                SnakeSegment,
                position.clone(),
            ));

            commands.spawn((
                SpriteBundle {
                    texture: assets.effect.clone(),
                    transform: transform.clone(),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(2.5, 2.5)),
                        ..default()
                    },
                    ..default()
                },
                GrowEffect {
                    timer: Timer::from_seconds(0.33, TimerMode::Once),
                },
            ));

            gameplay_channel
                .play(audio_assets.eat.clone())
                .with_volume(0.5);
        }
    }
}

pub fn delete_grow_effect_system(
    time: Res<Time>,
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut GrowEffect)>,
) {
    for (entity, mut enemy) in effect_query.iter_mut() {
        if enemy.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn snake_death_system(
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
    segment_query: Query<&Position, (With<SnakeSegment>, Without<SnakeHead>)>,
    head_query: Query<&Position, With<SnakeHead>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let segment_count = segment_query.iter().count();

    if segment_count <= 1 {
        let _ = game_state.set(GameState::GameOver);
    }

    let head_position = head_query.single();

    if !head_position.in_world() {
        let _ = game_state.set(GameState::GameOver);

        gameplay_channel.play(audio_assets.death_by_bumping.clone());
    }

    for segment_position in segment_query.iter() {
        if segment_position == head_position {
            let _ = game_state.set(GameState::GameOver);
            gameplay_channel.play(audio_assets.death_by_bumping.clone());
        }
    }
}
