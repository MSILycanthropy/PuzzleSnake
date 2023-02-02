use bevy::prelude::*;
use rand::Rng;

use crate::{level::LEVEL_SIZE, GameState, Position, TextureAssets};

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

#[derive(Component)]
pub struct Food;

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

    commands.insert_resource(LastTailPosition(None));
}

pub fn snake_input_system(
    mut head_query: Query<&mut SnakeHead>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut head = head_query.single_mut();

    // TODO: A bit of weirdness exists if you try and go mad fast with the key presses
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
    mut last_tail_position: ResMut<LastTailPosition>,
    mut head_query: Query<(&mut Position, &SnakeHead)>,
    mut segments_query: Query<&mut Position, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
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

        transform.translation = new_translation;
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

pub fn rotate_snake_head_system(mut head_query: Query<(&mut Transform, &SnakeHead)>) {
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
    let last_segment = segments_query.iter_mut().last().unwrap();

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

pub fn spawn_food_system(
    mut commands: Commands,
    food_query: Query<Entity, With<Food>>,
    assets: Res<TextureAssets>,
) {
    if food_query.iter().count() > 0 {
        return;
    }

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-LEVEL_SIZE..LEVEL_SIZE);
    let y = rng.gen_range(-LEVEL_SIZE..LEVEL_SIZE);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.wizard_sheet.clone(),
            transform: Transform::from_xyz(x as f32, y as f32, 1.0),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        Food,
        Position { x, y },
    ));
}

// TODO: Spawn the tail with the correct rotation.
pub fn snake_growth_system(
    mut commands: Commands,
    food_query: Query<(Entity, &Position), With<Food>>,
    head_query: Query<&Position, With<SnakeHead>>,
    tail_query: Query<Entity, With<SnakeTail>>,
    last_tail_position: Res<LastTailPosition>,
    assets: Res<TextureAssets>,
) {
    let head_position = head_query.single();

    for (food_entity, food_position) in food_query.iter() {
        if let Some(last_tail_position) = last_tail_position.0 {
            if food_position == head_position {
                for entity in tail_query.iter() {
                    commands.entity(entity).remove::<SnakeTail>();
                }

                commands.entity(food_entity).despawn();

                commands.spawn((
                    SpriteBundle {
                        texture: assets.tail.clone(),
                        transform: Transform::from_xyz(
                            last_tail_position.x as f32,
                            last_tail_position.y as f32,
                            1.0,
                        ),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(1., 1.)),
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

    if head_position.x > LEVEL_SIZE
        || head_position.x < -LEVEL_SIZE
        || head_position.y > LEVEL_SIZE
        || head_position.y < -LEVEL_SIZE
    {
        println!("You died!");

        let _ = game_state.set(GameState::GameOver);
    }

    for segment_position in segment_query.iter() {
        if segment_position == head_position {
            println!("You died!");

            let _ = game_state.set(GameState::GameOver);
        }
    }
}
