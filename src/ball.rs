use bevy::prelude::*;
use rand::prelude::*;

use crate::startup::*;


pub const BALL_SIZE: f32 = 5.0;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
pub struct Ball {
    pub direction: Vec2,
    pub speed: f32,
}


pub fn ball_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Ball)>
    ) {
    let Ok((mut transform, mut ball)) = query.single_mut() else {
        panic!("Query not found... Exiting");
    };
    transform.translation.x += ball.direction.x;
    transform.translation.y += ball.direction.y;

    let y_max = SCREEN_HEIGHT/2.0;
    let x_max = SCREEN_WIDTH/2.0;

    if transform.translation.x < -x_max {
        transform.translation.x = -x_max;
        ball.direction.x *= -1.0;
    } else if transform.translation.x > x_max {
        transform.translation.x = x_max;
        ball.direction.x *= -1.0;
    }

    if transform.translation.y < -y_max {
        transform.translation.y = -y_max;
        ball.direction.y *= -1.0;
    } else if transform.translation.y > y_max {
        transform.translation.y = y_max;
        ball.direction.y *= -1.0;
    }

    // // Clamping the ball to the bounds
    // transform.translation.x = transform.translation.x.clamp(-max_x, max_x);
    // transform.translation.y = transform.translation.y.clamp(-max_y, max_y);
    
}
