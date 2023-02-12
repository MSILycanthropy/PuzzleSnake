use bevy::{ecs::system::Command, prelude::*};
use bevy_kira_audio::prelude::*;
use iyes_loopless::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{
    despawn, level::LEVEL_SIZE, music::Gameplay, snake::Snake, AudioAssets, GameState, Position,
    TextureAssets,
};

const MAX_ENEMIES: usize = 1;
const ENEMY_DECISION_TIME_MIN: f32 = 0.5;
const ENEMY_DECISION_TIME_MAX: f32 = 1.5;
const ENEMY_ATTACK_SPEED: f32 = 5.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MaxEnemies>()
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::Playing)
                    .with_system(spawn_enemy_system)
                    .with_system(enemy_state_management_system)
                    .with_system(move_enemy_system)
                    .with_system(map_enemy_position)
                    .with_system(enemy_attack_animation_system)
                    .with_system(enemy_attack_system)
                    .with_system(enemy_attack_move_system)
                    .into(),
            )
            .add_exit_system(GameState::Playing, despawn::<Enemy>)
            .add_exit_system(GameState::Playing, despawn::<EnemyAttack>);
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct MaxEnemies(pub usize);

impl Default for MaxEnemies {
    fn default() -> Self {
        MaxEnemies(MAX_ENEMIES)
    }
}

#[derive(Component)]
pub struct Enemy {
    decision_timer: Timer,
    atk_anim_timer: Timer,
    move_step_timer: Timer,
}

impl Enemy {
    fn reset_decision_timer(&mut self) {
        let mut rng = rand::thread_rng();

        self.decision_timer = Timer::from_seconds(
            rng.gen_range(ENEMY_DECISION_TIME_MIN..ENEMY_DECISION_TIME_MAX),
            TimerMode::Once,
        );
    }

    fn reset_attack_animation_timer(&mut self, enemy_type: &EnemyType) {
        let time = match enemy_type {
            EnemyType::Wizard => 0.5,
            EnemyType::Knight => 2.5,
        };

        self.atk_anim_timer = Timer::from_seconds(time, TimerMode::Once);
    }
}

impl From<EnemyType> for Enemy {
    fn from(enemy_type: EnemyType) -> Self {
        let mut rng = rand::thread_rng();

        match enemy_type {
            EnemyType::Wizard => Enemy {
                decision_timer: Timer::from_seconds(
                    rng.gen_range(ENEMY_DECISION_TIME_MIN..ENEMY_DECISION_TIME_MAX),
                    TimerMode::Once,
                ),
                atk_anim_timer: Timer::from_seconds(0.5, TimerMode::Once),
                move_step_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            },
            EnemyType::Knight => Enemy {
                decision_timer: Timer::from_seconds(
                    rng.gen_range(
                        (ENEMY_DECISION_TIME_MIN / 1.25)..(ENEMY_DECISION_TIME_MAX / 1.25),
                    ),
                    TimerMode::Once,
                ),
                atk_anim_timer: Timer::from_seconds(2.5, TimerMode::Once),
                move_step_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            },
        }
    }
}

struct SpawnEnemy;

impl Command for SpawnEnemy {
    fn write(self, world: &mut World) {
        let snake = world.get_resource::<Snake>().unwrap().clone();

        let mut position = random_position();
        while snake.segments.contains(&position) {
            position = random_position()
        }

        let assets = world.get_resource::<TextureAssets>().unwrap();

        let enemy_type = match rand::random::<bool>() {
            true => EnemyType::Wizard,
            false => EnemyType::Knight,
        };

        world.spawn((
            Enemy::from(enemy_type.clone()),
            position,
            SpriteSheetBundle {
                texture_atlas: match enemy_type {
                    EnemyType::Wizard => assets.wizard_sheet.clone(),
                    EnemyType::Knight => assets.knight_sheet.clone(),
                },
                transform: Transform::from_xyz(position.x as f32, position.y as f32, 1.5),
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            Target(None),
            EnemyState::Idle,
            enemy_type,
        ));
    }
}

#[derive(Component, Clone, PartialEq, Eq)]
enum EnemyType {
    Knight,
    Wizard,
}

#[derive(Component, PartialEq, Eq, Clone, Debug)]
enum EnemyState {
    Idle,
    AttackAnimation,
    Attacking,
    Moving,
}

impl EnemyState {
    fn is_attack_animation(&self) -> bool {
        *self == EnemyState::AttackAnimation
    }

    fn is_attacking(&self) -> bool {
        *self == EnemyState::Attacking
    }

    fn is_moving(&self) -> bool {
        *self == EnemyState::Moving
    }

    fn is_idle(&self) -> bool {
        *self == EnemyState::Idle
    }

    fn to_idle(&mut self) {
        *self = EnemyState::Idle;
    }

    fn to_attacking(&mut self) {
        *self = EnemyState::Attacking;
    }

    fn randomize() -> Self {
        let mut rng = rand::thread_rng();
        let states = [
            EnemyState::Idle,
            EnemyState::AttackAnimation,
            EnemyState::Moving,
        ];
        states
            .choose_weighted(&mut rng, |state| match state {
                EnemyState::Idle => 5,
                EnemyState::AttackAnimation => 2,
                EnemyState::Moving => 3,
                _ => 0,
            })
            .unwrap()
            .clone()
    }
}

#[derive(Component)]
pub struct EnemyAttack {
    direction: Vec2,
}

#[derive(Component, Deref, DerefMut)]
struct Target(Option<Position>);

fn random_position() -> Position {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-LEVEL_SIZE.x..=LEVEL_SIZE.x);
    let y = rng.gen_range(-LEVEL_SIZE.y..=LEVEL_SIZE.y);
    Position { x, y }
}

fn spawn_enemy_system(
    mut commands: Commands,
    enemies: Query<&Enemy>,
    max_enemies: Res<MaxEnemies>,
) {
    let enemies_count = enemies.iter().count();

    if enemies_count >= max_enemies.0 {
        return;
    }

    commands.add(SpawnEnemy);
}

fn enemy_state_management_system(
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
    mut enemy_query: Query<(
        &mut Enemy,
        &EnemyType,
        &mut EnemyState,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut enemy, enemy_type, mut enemy_state, mut sprite) in enemy_query.iter_mut() {
        if !enemy_state.is_idle() {
            continue;
        }

        if !enemy.decision_timer.tick(time.delta()).just_finished() {
            continue;
        }

        let new_state = EnemyState::randomize();

        match new_state {
            EnemyState::Idle => {
                sprite.index = 0;
                enemy.reset_decision_timer()
            }
            EnemyState::AttackAnimation => match enemy_type {
                EnemyType::Wizard => {
                    sprite.index = 2;
                    gameplay_channel
                        .play(audio_assets.wizard_prepare.clone())
                        .with_volume(0.25);
                }
                EnemyType::Knight => {}
            },
            _ => {}
        }

        *enemy_state = new_state;
    }
}

fn move_enemy_system(
    time: Res<Time>,
    mut enemy_query: Query<(
        &mut Enemy,
        &mut EnemyState,
        &EnemyType,
        &mut Target,
        &mut Position,
        &mut TextureAtlasSprite,
    )>,
    snake: Res<Snake>,
) {
    for (mut enemy, mut enemy_state, enemy_type, mut target, mut position, mut sprite) in
        enemy_query.iter_mut()
    {
        if !enemy_state.is_moving() {
            continue;
        }

        if target.is_none() {
            match enemy_type {
                _ => {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(-LEVEL_SIZE.x..LEVEL_SIZE.x);
                    let y = rng.gen_range(-LEVEL_SIZE.y..LEVEL_SIZE.y);

                    *target = Target(Some(Position { x, y }));
                }
            }

            continue;
        }

        if !enemy.move_step_timer.tick(time.delta()).just_finished() {
            continue;
        }

        let target_position = target.unwrap();

        if *position == target_position {
            *target = Target(None);
            enemy_state.to_idle();
            enemy.reset_decision_timer();
            continue;
        }

        let old_position = position.clone();

        if position.x < target_position.x {
            position.x += 1;
            sprite.index = 1;
        } else if position.x > target_position.x {
            position.x -= 1;
            sprite.index = 0;
        }

        if position.y < target_position.y {
            position.y += 1;
        } else if position.y > target_position.y {
            position.y -= 1;
        }

        let any_segments_in_position = snake
            .segments
            .iter()
            .any(|segment_position| *segment_position == *position);

        if any_segments_in_position {
            *target = Target(None);
            *position = old_position;
            enemy_state.to_idle();
            enemy.reset_decision_timer();
        }
    }
}

fn map_enemy_position(mut enemies: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in enemies.iter_mut() {
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    }
}

fn enemy_attack_animation_system(
    time: Res<Time>,
    snake: Res<Snake>,
    mut enemy_query: Query<(
        &mut Enemy,
        &mut EnemyState,
        &EnemyType,
        &Position,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut enemy, mut enemy_state, enemy_type, position, mut sprite) in enemy_query.iter_mut() {
        if !enemy_state.is_attack_animation() {
            continue;
        }

        if !enemy.atk_anim_timer.tick(time.delta()).just_finished() {
            for segment in snake.segments.iter() {
                // If there is a segment directly below the knight, attack
                if segment.x == position.x && segment.y == position.y - 1 {
                    // gameplay_channel.play(audio_assets.knight_attack.clone());
                    enemy_state.to_attacking();
                    enemy.reset_attack_animation_timer(enemy_type);
                }
            }

            continue;
        }

        enemy_state.to_attacking();
        enemy.reset_attack_animation_timer(enemy_type);
        sprite.index = 0;
    }
}

fn enemy_attack_system(
    assets: Res<TextureAssets>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
    mut snake: ResMut<Snake>,
    mut commands: Commands,
    mut enemy_query: Query<
        (
            &mut Enemy,
            &mut EnemyState,
            &mut TextureAtlasSprite,
            &Position,
            &EnemyType,
            &Transform,
        ),
        With<Enemy>,
    >,
) {
    for (mut enemy, mut enemy_state, mut sprite, position, enemy_type, transform) in
        enemy_query.iter_mut()
    {
        println!("Enemy state: {:?}", enemy_state);

        if !enemy_state.is_attacking() {
            continue;
        }

        match enemy_type {
            EnemyType::Wizard => {
                let mut rng = rand::thread_rng();
                let segments = snake.segments.iter().collect::<Vec<_>>();
                let segment_position = segments.choose(&mut rng).unwrap();
                let mut transform = *transform;
                let direction = Vec2::new(
                    segment_position.x as f32 - transform.translation.x,
                    segment_position.y as f32 - transform.translation.y,
                )
                .normalize();

                transform.rotate(Quat::from_rotation_z(
                    Vec2::new(1., 1.).angle_between(direction),
                ));

                commands.spawn((
                    SpriteBundle {
                        texture: assets.projectile.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::new(0.5, 0.5)),
                            ..default()
                        },
                        transform,
                        ..default()
                    },
                    EnemyAttack { direction },
                ));

                gameplay_channel.play(audio_assets.wizard_attack.clone());
            }
            EnemyType::Knight => {
                sprite.index = 2;
                gameplay_channel.play(audio_assets.knight_attack.clone());
                let mut damaged = false;

                let segments = snake.segments.iter().collect::<Vec<_>>();
                for segment in segments {
                    // If there is a segment directly below the knight, attack
                    if segment.x == position.x && segment.y == position.y - 1 {
                        // gameplay_channel.play(audio_assets.knight_attack.clone());
                        damaged = true;
                    }
                }

                if damaged {
                    snake.damage(1);
                    if snake.is_dead() {
                        commands.insert_resource(NextState(GameState::GameOver));
                    }
                }
            }
        }

        enemy_state.to_idle();
        enemy.reset_decision_timer();
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
            commands.entity(entity).despawn();
        }
    }
}
