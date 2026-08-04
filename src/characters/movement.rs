use crate::characters::animation::*;
use crate::characters::config::{AnimationType, CharacterEntry};
use bevy::prelude::*;

fn read_movement_input(input: &ButtonInput<KeyCode>) -> Vec2 {
    const MOVEMENT_KEYS: [(KeyCode, Vec2); 8] = [
        // Arrow keys
        (KeyCode::ArrowLeft, Vec2::NEG_X),
        (KeyCode::ArrowRight, Vec2::X),
        (KeyCode::ArrowUp, Vec2::Y),
        (KeyCode::ArrowDown, Vec2::NEG_Y),
        // WASD
        (KeyCode::KeyA, Vec2::NEG_X),
        (KeyCode::KeyD, Vec2::X),
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyS, Vec2::NEG_Y),
    ];

    MOVEMENT_KEYS
        .iter()
        .filter(|(key, _)| input.pressed(*key))
        .map(|(_, direction)| *direction)
        .sum()
}

fn calculate_movement_speed(character: &CharacterEntry, is_running: bool) -> f32 {
    if is_running {
        character.base_move_speed * character.run_speed_multiplier
    } else {
        character.base_move_speed
    }
}

//Player marker
#[derive(Component)]
pub struct Player;
