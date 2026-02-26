use bevy::prelude::*;

use crate::ball::*;
use crate::startup::*;
use crate::collision::*;

pub const RECTANGLE_HEIGHT: f32 = 100.0;
pub const RECTANGLE_WIDTH: f32 = 10.0;

#[derive(Component)]
#[require(
    Position,
    Collider = Collider(Rectangle::new(RECTANGLE_WIDTH, RECTANGLE_HEIGHT))
)]
pub struct Paddle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

pub fn move_paddles(
    mut player_paddle: Single<&mut Transform, (With<Player>, Without<Ai>)>,
    mut ai_paddle: Single<&mut Transform, (With<Ai>, Without<Player>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let max_y = (SCREEN_HEIGHT / 2.0) - (RECTANGLE_HEIGHT / 2.0);
    let min_y = -(SCREEN_HEIGHT / 2.0) + (RECTANGLE_HEIGHT / 2.0);

    // let &mut paddle_1 = Query::single(&player_paddle).unwrap();
    // let &mut paddle_2 = Query::single(&ai_paddle).unwrap();


    // Player paddle movement
    if keyboard_input.pressed(KeyCode::KeyW) {
        player_paddle.translation.y += 12.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        player_paddle.translation.y -= 12.0;
    }

    player_paddle.translation.y = player_paddle.translation.y.clamp(min_y, max_y);

    // Ai paddle movement
    if keyboard_input.pressed(KeyCode::KeyO) {
        ai_paddle.translation.y += 12.0;
    }
    if keyboard_input.pressed(KeyCode::KeyL) {
        ai_paddle.translation.y -= 12.0;
    }

    ai_paddle.translation.y = ai_paddle.translation.y.clamp(min_y, max_y);
}

// TODO: Complete paddle_collision
// pub fn paddle_collision(
//     mut paddle_query: Query<&mut Transform, With<Paddle>>,
//     mut ball_query: Query<&mut Transform, With<Ball>>
// ) {
//
// }
