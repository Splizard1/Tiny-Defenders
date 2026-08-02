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

// Animation configuration
#[derive(Component)]
pub struct AnimationController {
    pub current_animation: AnimationType,
    pub facing: Facing,
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_animation: Animationtype::Walk,
            facing: Facing::Down,
        }
    }
}

#[derive(Component, Default)]
pub struct AnimatioNState {
    pub is_moving: bool,
    pub was_moving: bool,
    pub is_jumping: bool,
    pub was_jumping: bool,
}

#[derive(Component, Default)]
pub struct Animationtimer(pub Timer);
