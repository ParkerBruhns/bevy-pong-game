use bevy::prelude::*;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::window::{PresentMode, WindowMode};

use crate::ball::*;
use crate::paddles::*;
use crate::startup::*;

#[derive(Component, Default)]
pub struct Collider(pub Rectangle);

impl Collider {
    fn half_size(&self) -> Vec2 {
        self.0.half_size
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Collision {
    Top,
    Bottom,
    Left,
    Right,
}

pub fn handle_collisions(
    ball: Single<(&Position, &Collider), With<Ball>>,
    other_things: Query<(&Position, &Collider), Without<Ball>>
) {
    let (ball_position, ball_collider) = ball.into_inner();

    for (other_position, other_collider) in &other_things {
        if let Some(collision) = collide_with_side(
            Aabb2d::new(ball_position.0, ball_collider.half_size()),
            Aabb2d::new(other_position.0, other_collider.half_size())
        ) {
            match collision {
                Collision::Left || Collision::Right -> ball.reverse_x_direction(),
                Collision::Top || Collision::Bottom -> ball.reverse_y_direction(),
            }
        }
    }

}

pub fn collide_with_side(ball: Aabb2d, wall: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&wall) {
        return None;
    }

    let closest_point = wall.closest_point(ball.center());
    let offset = ball.center() - closest_point;

    let side = if offset.x.abs() > offset.y.abs() {
        if offset.x < 0. {
            Collision::Left
        } else {
            Collision::Right
        }
    } else if offset.y > 0. {
        Collision::Top
    } else {
        Collision::Bottom
    };

    Some(side)
}
