use crate::characters::animations::*;
use crate::characters::config::{AnimationType, CharacterEntry, CharactersList};
use crate::characters::movement::Player;
use bevy::prelude::*;

const PLAYER_SCALE: f32 = 0.8;
const PLAYER_Z_POSITION: f32 = 20.0;

#[derive(Resource, Default)]
pub struct CurrentCharacterIndex {
    pub index: usize,
}

#[derive(Resource)]
pub struct CharactersListResource {
    pub handle: Handle<CharactersList>,
}

#[derive(Resource)]
pub struct PendingCharacterSwitch {
    pub index: usize,
    pub character: CharacterEntry,
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

// Creates a texture atlas layout for a character
fn create_character_atlas_layout(
    atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    character_entry: &CharacterEntry,
) -> Handle<TextureAtlasLayout> {
    let max_row = character_entry.calculate_max_animation_row();

    atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(character_entry.tile_size),
        character_entry.atlas_columns as u32,
        (max_row + 1) as u32,
        None,
        None,
    ))
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut character_index: ResMut<CurrentCharacterIndex>,
) {
    // Load the characters list
    let characters_list_handle: Handle<CharactersList> =
        asset_server.load("characters/characters.ron");

    // Store the handle in a resource
    commands.insert_resource(CharactersListResource {
        handle: characters_list_handle,
    });

    // Initialize with first character
    character_index.index = 0;

    // Spawn player entity (will be initialized once asset loads)
    commands.spawn((
        Player,
        Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z_POSITION))
            .with_scale(Vec3::splat(PLAYER_SCALE)),
        Sprite::default(),
    ));
}

pub fn initialize_player_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    characters_lists: Res<Assets<CharactersList>>,
    character_index: Res<CurrentCharacterIndex>,
    characters_list_res: Option<Res<CharactersListResource>>,
    mut query: Query<Entity, (With<Player>, Without<AnimationController>)>,
) {
    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    for entity in query.iter_mut() {
        let Some(characters_list) = characters_lists.get(&characters_list_res.handle) else {
            continue;
        };

        if character_index.index >= characters_list.characters.len() {
            continue;
        };

        let character_entry = &characters_list.characters[character_index.index];

        let texture = asset_server.load(&character_entry.texture_path);
        let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);

        let sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

        commands.entity(entity).insert((
            AnimationController::default(),
            AnimationState::default(),
            AnimationTimer(Timer::from_seconds(
                ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
            character_entry.clone(),
            sprite,
        ));
    }
}

pub fn switch_character(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    characters_lists: Res<Assets<CharactersList>>,
    characters_list_res: Option<Res<CharactersListResource>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    const DIGIT_KEYS: [KeyCode; 9] = [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
        KeyCode::Digit6,
        KeyCode::Digit7,
        KeyCode::Digit8,
        KeyCode::Digit9,
    ];

    let Some(new_index) = DIGIT_KEYS.iter().position(|key| input.just_pressed(*key)) else {
        return;
    };

    let Some(characters_list_res) = characters_list_res else {
        return;
    };

    let Some(characters_list) = characters_lists.get(&characters_list_res.handle) else {
        return;
    };

    let Some(character_entry) = characters_list.characters.get(new_index) else {
        return;
    };

    // Start loading the texture, but keep the current sprite visible.
    let texture: Handle<Image> = asset_server.load(character_entry.texture_path.clone());

    let layout = create_character_atlas_layout(&mut atlas_layouts, character_entry);

    commands.insert_resource(PendingCharacterSwitch {
        index: new_index,
        character: character_entry.clone(),
        texture,
        layout,
    });
}

pub fn apply_character_switch(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    pending: Option<Res<PendingCharacterSwitch>>,
    mut character_index: ResMut<CurrentCharacterIndex>,
    mut query: Query<
        (
            &mut CharacterEntry,
            &mut Sprite,
            &mut AnimationController,
            &mut AnimationState,
            &mut AnimationTimer,
        ),
        With<Player>,
    >,
) {
    let Some(pending) = pending else {
        return;
    };

    // Keep displaying the old character until the new image is ready.
    if !asset_server.is_loaded_with_dependencies(pending.texture.id()) {
        return;
    }

    let Ok((mut current_entry, mut sprite, mut animated, mut state, mut timer)) =
        query.single_mut()
    else {
        return;
    };

    animated.current_animation = AnimationType::Walk;

    // Start on the correct directional row, rather than always atlas index 0.
    let start_index = animated
        .get_clip(&pending.character)
        .map_or(0, |clip| clip.start());

    *current_entry = pending.character.clone();

    *sprite = Sprite::from_atlas_image(
        pending.texture.clone(),
        TextureAtlas {
            layout: pending.layout.clone(),
            index: start_index,
        },
    );

    state.is_moving = false;
    state.was_moving = false;
    state.is_jumping = false;
    state.was_jumping = false;

    if let Some(walk_definition) = pending.character.animations.get(&AnimationType::Walk) {
        timer.0.set_duration(std::time::Duration::from_secs_f32(
            walk_definition.frame_time,
        ));
    }

    timer.0.reset();
    character_index.index = pending.index;

    commands.remove_resource::<PendingCharacterSwitch>();
}
