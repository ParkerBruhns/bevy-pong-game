use bevy::{prelude::*, text::cosmic_text::Angle};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::startup::*;

pub const BALL_SIZE: f32 = 5.0;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(Vec2);

#[derive(Component)]
#[require(Position)]
pub struct Ball {
    pub angle: f32,
    pub direction: Vec3,
    pub speed: f32,
}

impl Ball {
    pub fn new_ball(angle: f32, speed: f32) -> Ball {
        return Ball {
            angle: angle, 
            direction: Vec3::new(angle.cos(), angle.sin(), 0.0), 
            speed: speed 
        }
    }
}

pub fn ball_movement(
    mut _commands: Commands,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Ball)>,
) {
    let Ok((mut transform, mut ball)) = query.single_mut() else {
        panic!("Query not found... Exiting");
    };

    transform.translation += ball.direction * ball.speed * time.delta_secs();

    let y_max = SCREEN_HEIGHT / 2.0;
    let x_max = SCREEN_WIDTH / 2.0;

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
}
