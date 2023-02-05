use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use rand::{seq::SliceRandom, Rng};

use crate::{
    despawn,
    level::LEVEL_SIZE,
    music::Gameplay,
    snake::{GrowEffect, SnakeHead, SnakeSegment, SnakeTail},
    AudioAssets, GameState, Position, TextureAssets,
};

const ENEMY_COUNT: usize = 3;
const ENEMY_DECISION_TIME_MIN: f32 = 0.5;
const ENEMY_DECISION_TIME_MAX: f32 = 2.0;
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
                // .with_system(move_enemy_system)
                .with_system(enemy_movement_system)
                .with_system(map_enemy_world_position.after(enemy_movement_system))
                .with_system(enemy_state_management_system)
                .with_system(enemy_attack_system)
                .with_system(enemy_attack_animation_system)
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
    decision_timer: Timer,
    atk_anim_timer: Timer,
    move_timer: Timer,
}

impl Enemy {
    fn reset_decision_timer(&mut self) {
        let mut rng = rand::thread_rng();

        self.decision_timer = Timer::from_seconds(
            rng.gen_range(ENEMY_DECISION_TIME_MIN..ENEMY_DECISION_TIME_MAX),
            TimerMode::Once,
        );
    }

    fn reset_attack_animation_timer(&mut self) {
        self.atk_anim_timer = Timer::from_seconds(0.75, TimerMode::Once);
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
                move_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
            },
        }
    }
}

#[derive(Component)]
enum EnemyType {
    Wizard,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    sprite_sheet: SpriteSheetBundle,
    enemy_type: EnemyType,
    position: Position,
    target: Target,
    state: EnemyState,
}

#[derive(Component)]
struct EnemyAttack {
    direction: Vec2,
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
        states.choose(&mut rng).unwrap().clone()
    }
}

#[derive(Component, Deref, DerefMut)]
struct Target(Option<Position>);

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

    commands.spawn(EnemyBundle {
        enemy: Enemy::from(EnemyType::Wizard),
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: assets.wizard_sheet.clone(),
            transform: Transform::from_xyz(x as f32, y as f32, 1.0),
            sprite: TextureAtlasSprite {
                custom_size: Some(Vec2::new(1., 1.)),
                ..default()
            },
            ..default()
        },
        enemy_type: EnemyType::Wizard,
        position: Position { x, y },
        target: Target(None),
        state: EnemyState::Idle,
    });
}

fn enemy_state_management_system(
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
    mut enemy_query: Query<(&mut Enemy, &mut EnemyState, &mut TextureAtlasSprite)>,
) {
    for (mut enemy, mut enemy_state, mut sprite) in enemy_query.iter_mut() {
        if !enemy_state.is_idle() {
            continue;
        }

        if !enemy.decision_timer.tick(time.delta()).just_finished() {
            continue;
        }

        let new_state = EnemyState::randomize();

        match new_state {
            EnemyState::Idle => enemy.reset_decision_timer(),
            EnemyState::AttackAnimation => {
                sprite.index = 2;
                gameplay_channel
                    .play(audio_assets.wizard_prepare.clone())
                    .with_volume(0.25);
            }
            _ => {}
        }

        *enemy_state = new_state;
    }
}

fn map_enemy_world_position(mut enemy_query: Query<(&Position, &mut Transform), With<Enemy>>) {
    // TODO: Consider lerping maybe?
    for (position, mut transform) in enemy_query.iter_mut() {
        transform.translation.x = position.x as f32;
        transform.translation.y = position.y as f32;
    }
}

fn enemy_attack_animation_system(
    time: Res<Time>,
    mut enemy_query: Query<(&mut Enemy, &mut EnemyState, &mut TextureAtlasSprite)>,
) {
    for (mut enemy, mut enemy_state, mut sprite) in enemy_query.iter_mut() {
        if !enemy_state.is_attack_animation() {
            continue;
        }

        if !enemy.atk_anim_timer.tick(time.delta()).just_finished() {
            continue;
        }

        enemy_state.to_attacking();
        enemy.reset_attack_animation_timer();
        sprite.index = 0;
    }
}

fn enemy_attack_system(
    assets: Res<TextureAssets>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
    segments_query: Query<&Position, With<SnakeSegment>>,
    mut commands: Commands,
    mut enemy_query: Query<(&mut Enemy, &mut EnemyState, &EnemyType, &Transform), With<Enemy>>,
) {
    let mut rng = rand::thread_rng();
    let segments = segments_query.iter().collect::<Vec<_>>();
    let segment_position = segments.choose(&mut rng).unwrap();

    for (mut enemy, mut enemy_state, enemy_type, transform) in enemy_query.iter_mut() {
        if !enemy_state.is_attacking() {
            continue;
        }

        match enemy_type {
            EnemyType::Wizard => {
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
        }

        enemy.reset_decision_timer();
        enemy_state.to_idle();
    }
}

fn enemy_movement_system(
    time: Res<Time>,
    mut enemy_query: Query<
        (
            &mut Enemy,
            &mut EnemyState,
            &EnemyType,
            &mut Target,
            &mut Position,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
    segments_query: Query<&Position, (With<SnakeSegment>, Without<Enemy>)>,
) {
    for (mut enemy, mut enemy_state, enemy_type, mut target, mut position, mut sprite) in
        enemy_query.iter_mut()
    {
        if !enemy_state.is_moving() {
            continue;
        }

        if target.is_none() {
            match enemy_type {
                EnemyType::Wizard => {
                    let mut rng = rand::thread_rng();
                    let x = rng.gen_range(-LEVEL_SIZE..LEVEL_SIZE);
                    let y = rng.gen_range(-LEVEL_SIZE..LEVEL_SIZE);

                    *target = Target(Some(Position { x, y }));
                }
            }

            continue;
        }

        if !enemy.move_timer.tick(time.delta()).just_finished() {
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

        let any_segments_in_position = segments_query
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

// TODO: There's some weird issue with deleting them and where they update to
// a frame later. Not really sure why that happens. So I'm just masking it
// with a stupid effect lmao. It looks kinda stupid but... it works.
fn enemy_attack_damage_system(
    assets: Res<TextureAssets>,
    audio_assets: Res<AudioAssets>,
    gameplay_channel: Res<AudioChannel<Gameplay>>,
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

    let last_segment_transform = if let Some(last_segment) = segments_query.iter().last() {
        last_segment.1.clone()
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

                commands.spawn((
                    SpriteBundle {
                        texture: assets.effect.clone(),
                        transform: last_segment_transform.clone(),
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

                gameplay_channel.play(audio_assets.hit.clone());
                return;
            }
        }
    }
}
