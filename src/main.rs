#![allow(unused_imports, dead_code)]

mod ball;
mod paddles;
mod startup;
mod collision;

use bevy::prelude::*;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};
use bevy::window::{PresentMode, WindowMode};

use crate::ball::*;
use crate::paddles::*;
use crate::startup::*;
use crate::collision::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                mode: WindowMode::Windowed,
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (spawn_camera, spawn_ball, spawn_paddles, spawn_line),
        )
        .add_systems(Update, (move_paddles, ball_movement, handle_collisions))
        .run();
}
