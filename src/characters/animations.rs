use crate::characters::config::{AnimationType, CharacterConfig};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

// Animation timing - 10 fps is 0.1 seconds per frame
pub const ANIMATION_FRAME_TIME: f32 = 0.1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    // Convert a direction vector into a Facing enum
    pub fn from_direction(direction: Vec2) -> Self {
        if direction.x.abs() > direction.y.abs() {
            if direction.x > 0.0 {
                Facing::Right
            } else {
                Facing::Left
            }
        } else {
            if direction.y > 0.0 {
                Facing::Up
            } else {
                Facing::Down
            }
        }
    }

    fn direction_index(self) -> usize {
        match self {
            Facing::Up => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Right => 3,
        }
    }
}
