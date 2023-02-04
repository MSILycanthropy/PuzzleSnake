use bevy::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{
    despawn,
    level::LEVEL_SIZE,
    snake::{SnakeHead, SnakeSegment, SnakeTail},
    GameState, Position, TextureAssets,
};

const ENEMY_COUNT: usize = 3;
const ENEMY_MOVE_TIME: f32 = 0.875;
const ENEMY_ATTACK_TIME_MIN: f32 = 4.0;
const ENEMY_ATTACK_TIME_MAX: f32 = 8.0;
const ENEMY_ATTACK_SPEED: f32 = 3.0;
const ENEMY_ATTACK_DAMAGE_RADIUS: f32 = 0.75;

// TODO: This whole thing basically just takes wizards into account rn
// eventually we need stuff for other enemies
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(spawn_enemy_system)
                .with_system(move_enemy_system)
                .with_system(map_enemy_world_position.after(move_enemy_system))
                .with_system(enemy_attack_system.after(map_enemy_world_position))
                .with_system(enemy_attack_damage_system)
                .with_system(enemy_attack_move_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing)
                .with_system(despawn::<Enemy>)
                .with_system(despawn::<EnemyAttack>),
        );
    }
}

#[derive(Component)]
pub struct Enemy {
    move_timer: Timer,
    attack_timer: Timer,
}

#[derive(Component)]
struct EnemyAttack {
    direction: Vec2,
}

impl Enemy {
    fn will_move(&mut self, time: &Res<Time>) -> bool {
        self.move_timer.tick(time.delta()).just_finished()
    }

    fn will_attack(&mut self, time: &Res<Time>) -> bool {
        self.attack_timer.tick(time.delta()).just_finished()
    }

    fn reset_attack_timer(&mut self) {
        let mut rng = rand::thread_rng();
        self.attack_timer = Timer::from_seconds(
            rng.gen_range(ENEMY_ATTACK_TIME_MIN..ENEMY_ATTACK_TIME_MAX),
            TimerMode::Once,
        );
    }
}

fn spawn_enemy_system(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    assets: Res<TextureAssets>,
) {
    if enemy_query.iter().count() >= ENEMY_COUNT {
        return;
    }

    // TODO: Be smarter about this so we don't spawn enemies on top of the player
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
        Enemy {
            move_timer: Timer::from_seconds(ENEMY_MOVE_TIME, TimerMode::Repeating),
            attack_timer: Timer::from_seconds(
                rng.gen_range(ENEMY_ATTACK_TIME_MIN..ENEMY_ATTACK_TIME_MAX),
                TimerMode::Once,
            ),
        },
        Position { x, y },
    ));
}

// TODO: Make this a lot smarter. Right now it's just random movement.
// There is a pathfinding crate that could be used.

// ALso rn if an enemy ends up outside of the map.. the game just gets stuck in an infinite loop
fn move_enemy_system(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut Position), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();
    for (mut enemy, mut position) in enemy_query.iter_mut() {
        if !enemy.will_move(&time) {
            continue;
        }

        let x = rng.gen_range(-1..=1);
        let y = rng.gen_range(-1..=1);

        position.x += x;
        position.y += y;

        while !position.in_world() {
            let x = rng.gen_range(-1..=1);
            let y = rng.gen_range(-1..=1);

            position.x += x;
            position.y += y;
        }
    }
}

fn map_enemy_world_position(mut enemy_query: Query<(&Position, &mut Transform), With<Enemy>>) {
    // TODO: Consider lerping maybe?
    for (position, mut transform) in enemy_query.iter_mut() {
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    }
}

fn enemy_attack_system(
    time: Res<Time>,
    assets: Res<TextureAssets>,
    segments_query: Query<&Position, With<SnakeSegment>>,
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &Transform), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();
    let segments = segments_query.iter().collect::<Vec<_>>();
    let segment_position = segments.choose(&mut rng).unwrap();

    for (mut enemy, transform) in enemy_query.iter_mut() {
        if !enemy.will_attack(&time) {
            continue;
        }

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: assets.wizard_sheet.clone(),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(0.5, 0.5)),
                    ..default()
                },
                transform: *transform,
                ..default()
            },
            EnemyAttack {
                direction: Vec2::new(
                    segment_position.x as f32 - transform.translation.x,
                    segment_position.y as f32 - transform.translation.y,
                )
                .normalize(),
            },
        ));

        enemy.reset_attack_timer();
    }
}

fn enemy_attack_move_system(
    time: Res<Time>,
    mut commands: Commands,
    mut enemy_attack_query: Query<(Entity, &EnemyAttack, &mut Transform)>,
) {
    // Move in the direction of the target
    for (entity, enemy_attack, mut transform) in enemy_attack_query.iter_mut() {
        let movement_vector = enemy_attack.direction * ENEMY_ATTACK_SPEED * time.delta_seconds();

        transform.translation.x += movement_vector.x;
        transform.translation.y += movement_vector.y;

        if !Position::from(transform.translation.truncate()).in_world() {
            println!("Despawning enemy attack");
            commands.entity(entity).despawn();
            println!("Despawned enemy attack");
        }
    }
}

// TODO: There's some weird issue with deleting them and where they update to
// a frame later. Not really sure why that happens
// We might just wanna mask this with an animation, or something
fn enemy_attack_damage_system(
    mut commands: Commands,
    mut enemy_attack_query: Query<(Entity, &Transform), With<EnemyAttack>>,
    mut segments_query: Query<
        (Entity, &Transform),
        (With<SnakeSegment>, Without<SnakeTail>, Without<SnakeHead>),
    >,
) {
    let last_segment_entity = if let Some(last_segment) = segments_query.iter().last() {
        last_segment.0
    } else {
        return;
    };

    for (_, segment_transform) in segments_query.iter_mut() {
        for (enemy_attack_entity, enemy_attack_transform) in enemy_attack_query.iter_mut() {
            if enemy_attack_transform
                .translation
                .distance(segment_transform.translation)
                < ENEMY_ATTACK_DAMAGE_RADIUS
            {
                commands.entity(enemy_attack_entity).despawn();
                commands.entity(last_segment_entity).despawn();
                return;
            }
        }
    }
}
