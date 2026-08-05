use crate::characters::config::{AnimationType, CharacterEntry};
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
            Facing::Left => 1,
            Facing::Down => 2,
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

impl AnimationController {
    pub fn get_clip(&self, config: &CharacterEntry) -> Option<AnimationClip> {
        // Step 1. Get defenition of the current animation type
        let def = config.animations.get(&self.current_animation)?;
        //Step 2. Calc the row based on facing direction
        let row = if def.directional {
            def.start_row + self.facing.direction_index()
        } else {
            def.start_row
        };
        //Step 3. Create the AnimationClip
        Some(AnimationClip::new(
            row,
            def.frame_count,
            config.atlas_columns,
        ))
    }
}

impl Default for AnimationController {
    fn default() -> Self {
        Self {
            current_animation: AnimationType::Walk,
            facing: Facing::Down,
        }
    }
}

#[derive(Component, Default)]
pub struct AnimationState {
    pub is_moving: bool,
    pub was_moving: bool,
    pub is_jumping: bool,
    pub was_jumping: bool,
}

#[derive(Component, Default)]
pub struct AnimationTimer(pub Timer);

#[derive(Clone, Copy)]
pub struct AnimationClip {
    first: usize,
    last: usize,
}

impl AnimationClip {
    pub fn new(row: usize, frame_count: usize, atlas_columns: usize) -> Self {
        let first = row * atlas_columns;
        Self {
            first,
            last: first + frame_count - 1,
        }
    }

    pub fn start(self) -> usize {
        self.first
    }

    // Check if frame index belongs to this clip
    pub fn contains(self, index: usize) -> bool {
        (self.first..=self.last).contains(&index)
    }

    //Calculate the nxt frame, loop back
    pub fn next(self, index: usize) -> usize {
        if index >= self.last {
            self.first
        } else {
            index + 1
        }
    }

    pub fn is_complete(self, current_index: usize, timer_finished: bool) -> bool {
        current_index >= self.last && timer_finished
    }
}

pub fn animate_characters(
    time: Res<Time>,
    mut query: Query<(
        &AnimationController,
        &AnimationState,
        &mut AnimationTimer,
        &mut Sprite,
        &CharacterEntry,
    )>,
) {
    for (controller, state, mut timer, mut sprite, config) in &mut query {
        let Some(atlas) = sprite.texture_atlas.as_mut() else {
            continue;
        };

        let Some(clip) = controller.get_clip(config) else {
            continue;
        };

        let Some(anim_def) = config.animations.get(&controller.current_animation) else {
            continue;
        };

        if !clip.contains(atlas.index) {
            atlas.index = clip.start();
            timer.0.reset();
        }

        let just_started_moving = state.is_moving && !state.was_moving;
        let just_stopped_moving = !state.is_moving && state.was_moving;
        let just_started_jumping = state.is_jumping && !state.was_jumping;
        let just_stopped_jumping = !state.is_jumping && state.was_jumping;

        let should_animate = state.is_jumping || state.is_moving;

        let animation_changed = just_started_moving
            || just_stopped_moving
            || just_started_jumping
            || just_stopped_jumping;

        if animation_changed {
            atlas.index = clip.start();

            timer
                .0
                .set_duration(std::time::Duration::from_secs_f32(anim_def.frame_time));

            timer.0.reset();
        } else if should_animate {
            timer.0.tick(time.delta());

            if timer.0.just_finished() {
                atlas.index = clip.next(atlas.index);
            }
        } else {
            atlas.index = clip.start();
        }
    }
}

// Helper to update "was_moving" flags at the end of the frame
pub fn update_animation_flags(mut query: Query<&mut AnimationState>) {
    for mut state in query.iter_mut() {
        state.was_moving = state.is_moving;
        state.was_jumping = state.is_jumping;
    }
}
