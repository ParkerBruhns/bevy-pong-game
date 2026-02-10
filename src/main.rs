#![allow(unused_imports, dead_code)]

mod ball;
mod paddles;
mod startup;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowMode};
use bevy_rapier2d::prelude::*;

use crate::ball::*;
use crate::paddles::*;
use crate::startup::*;

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
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(
            Startup,
            (spawn_camera, spawn_ball, spawn_paddles, spawn_line),
        )
        .add_systems(Update, (move_paddles, ball_movement))
        .run();
}
