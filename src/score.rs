use bevy::prelude::*;

use crate::ball::*;
use crate::startup::*;
use crate::paddles::*;

#[derive(Resource)]
pub struct Score {
    pub player: u32,
    pub ai: u32,
}

#[derive(Event)]
pub struct Scored {
    entity: Entity,
}
