#![allow(unused_imports, dead_code)]

mod startup; 
mod ball;
mod paddles;

use bevy::prelude::*;
use bevy::window::{WindowMode, PresentMode};

use crate::startup::*;
use crate::ball::*;
use crate::paddles::*;

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
        .add_systems(Startup, (spawn_camera, spawn_ball, spawn_paddles, spawn_line))
        .add_systems(Update, (move_paddles, ball_movement))
        .run();
}
