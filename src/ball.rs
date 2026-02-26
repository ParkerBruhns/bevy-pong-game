use bevy::prelude::*;
use rand::prelude::*;

use crate::startup::*;
use crate::collision::*;

pub const BALL_SIZE: f32 = 8.0;

#[derive(Component, Default)]
#[require(Transform)]
pub struct Position(Vec2);

#[derive(Component)]
#[require(
    Position,
    Collider = Collider(Rectangle::new(BALL_SIZE * 2.0, BALL_SIZE * 2.0)))]
pub struct Ball {
    pub angle: f32,
    pub direction: Vec2,
    pub speed: f32,
}

impl Ball {
    pub fn new_ball(angle: f32, speed: f32) -> Self {
        Self {
            angle: angle, 
            direction: Vec2::new(angle.cos(), angle.sin()), 
            speed: speed 
        }
    }

    pub fn reverse_x_direction(&mut self) {
        self.direction.x *= -1.0;
    }
    
    pub fn reverse_y_direction(&mut self) {
        self.direction.y *= -1.0;
    }
    
    pub fn reverse_direction(&mut self) {
        self.reverse_x_direction();
        self.reverse_y_direction();
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

    transform.translation += ball.direction.extend(0.0) * ball.speed * time.delta_secs();

    let y_max = SCREEN_HEIGHT / 2.0;
    let x_max = SCREEN_WIDTH / 2.0;

    if transform.translation.x < -x_max {
        // transform.translation.x = -x_max;
        ball.reverse_x_direction();
    } else if transform.translation.x > x_max {
        // transform.translation.x = x_max;
        ball.reverse_x_direction();
    }

    if transform.translation.y < -y_max {
        // transform.translation.y = -y_max;
        ball.reverse_y_direction();
    } else if transform.translation.y > y_max {
        // transform.translation.y = y_max;
        ball.reverse_y_direction();
    }
}
