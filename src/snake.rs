use bevy::prelude::*;

#[derive(Component)]
struct Player {
    head: Option<SnakeHead>,
    tail: SnakeTail,
    segements: Vec<SnakeSegment>,
}

#[derive(Component)]
struct SnakeHead {
    direction: SnakeDirection,
}

#[derive(Component)]
struct SnakeTail;

#[derive(Component)]
struct SnakeSegment;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn to_world(&self) -> Vec3 {
        Vec3::new(self.x as f32 * SCALE, self.y as f32 * SCALE, 0.0)
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

fn snake_input_system(keyboard_input: Res<Input<KeyCode>>, mut player: Query<&mut Player>) {
    let mut player = player.single_mut();

    if let Some(mut head) = player.head {
        let dir = if keyboard_input.just_pressed(KeyCode::W) {
            SnakeDirection::Up
        } else if keyboard_input.just_pressed(KeyCode::S) {
            SnakeDirection::Down
        } else if keyboard_input.just_pressed(KeyCode::A) {
            SnakeDirection::Left
        } else if keyboard_input.just_pressed(KeyCode::D) {
            SnakeDirection::Right
        } else {
            head.direction;
        };

        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

fn snake_movement_system(mut player_query: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    let (mut transform, mut player) = player_query.single_mut();

    if let Some(head) = player.head {
        let dir = head.direction.to_vec();
        transform.translation += dir * PLAYER_SPEED * time.delta_seconds();
    }
}
