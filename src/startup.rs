use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::text::cosmic_text::ttf_parser::math::MathValue;
use bevy::math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume};

use rand::prelude::*;

use crate::ball::*;
use crate::paddles::*;

pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn_empty().insert(Camera2d);
}

// Startup Systems
pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning Ball...");
    let shape = Circle::new(BALL_SIZE);
    let color = Color::srgb(1.0, 1.0, 1.0);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    let mut rng = rand::rng();

    let min: f32 = 0.0;
    let max: f32 = std::f32::consts::PI;

    let angle = rng.random_range(min..max);
    let ball = Ball::new_ball(angle, 500.0);

    commands.spawn((
        ball,
        Transform::default(),
        Mesh2d(mesh),
        MeshMaterial2d(material),
    ));
}

pub fn spawn_paddles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(RECTANGLE_WIDTH, RECTANGLE_HEIGHT);
    let color = Color::srgb(0.9, 0.9, 0.9);

    let mesh = meshes.add(shape);
    let material = materials.add(color);
    // let transform: Transform = Transform::from_xyz(10.0, 10.0, 0.0);

    commands.spawn((
        Player,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(-SCREEN_WIDTH / 2.1, 0.0, 0.0),
    ));

    commands.spawn((
        Ai,
        Paddle,
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(SCREEN_WIDTH / 2.1, 0.0, 0.0),
    ));
}

pub fn spawn_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = Rectangle::new(RECTANGLE_WIDTH / 2.0, RECTANGLE_HEIGHT * 1.5);
    let color = Color::srgb(1.0, 1.0, 1.0);

    let mesh = meshes.add(shape);
    let material = materials.add(color);

    let mut line_height = SCREEN_HEIGHT;

    for _num in 1..20 {
        commands.spawn((
            Mesh2d(mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_xyz(0.0, line_height, 0.0),
        ));
        line_height = line_height - (SCREEN_HEIGHT / 10.0);
    }
}
