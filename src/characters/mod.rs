pub mod animations;
pub mod config;
pub mod movement;
pub mod spawn;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use config::CharactersList;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CharactersList>::new(&["characters.ron"]))
            .init_resource::<spawn::CurrentCharacterIndex>()
            .add_systems(Startup, spawn::spawn_player)
            .add_systems(
                Update,
                (
                    spawn::initialize_player_character,
                    spawn::switch_character,
                    spawn::apply_character_switch,
                    movement::move_player,
                    animations::animate_characters,
                    animations::update_animation_flags,
                    movement::update_jump_state,
                )
                    .chain(),
            );
    }
}
