use bevy::prelude::*;

use crate::startup::*;
use crate::ball::*;

#[derive(Component)]
#[require(Position)]
pub struct Paddle;
pub const RECTANGLE_HEIGHT: f32 = 48.0;
pub const RECTANGLE_WIDTH: f32 = 6.0;

pub fn move_paddles(
    mut paddles: Query<&mut Transform, With<Paddle>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
) {

    let max_y = (SCREEN_HEIGHT / 2.0) - (RECTANGLE_HEIGHT / 2.0);
    let min_y = -(SCREEN_HEIGHT / 2.0) + (RECTANGLE_HEIGHT / 2.0);

    let mut iter = paddles.iter_mut();
    if let Some(mut paddle_1) = iter.next() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            paddle_1.translation.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            paddle_1.translation.y -= 10.0;
        }

        paddle_1.translation.y = paddle_1.translation.y.clamp(min_y, max_y);
    }

    if let Some(mut paddle_2) = iter.next() {
        if keyboard_input.pressed(KeyCode::KeyO) {
            paddle_2.translation.y += 10.0;
        }
        if keyboard_input.pressed(KeyCode::KeyL) {
            paddle_2.translation.y -= 10.0;
        }

        paddle_2.translation.y = paddle_2.translation.y.clamp(min_y, max_y);
    }
}
