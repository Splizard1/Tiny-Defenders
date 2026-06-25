use bevy::prelude::*;
use bevy_procedural_tilemaps::prelude::*;

use crate::map::{
    assets::{load_assets, prepare_tilemap_handles},
    rules::build_world,
};

// -----------------  Configurable values ---------------------------
/// Modify these values to control the map size.
pub const GRID_X: u32 = 25;
pub const GRID_Y: u32 = 18;

// ------------------------------------------------------------------

const ASSETS_PATH: &str = "tile_layers";
const TILEMAP_FILE: &str = "tilemap.png";
/// Size of a block in world units (in Bevy 2d, 1 pixel is 1 world unit)
pub const TILE_SIZE: f32 = 32.;
/// Size of a grid node in world units
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, 1.);

const ASSETS_SCALE: Vec3 = Vec3::ONE;
/// Number of z layers in the map, derived from the default terrain layers.
const GRID_Z: u32 = 1;

pub fn map_pixel_dimensions() -> Vec2 {
    Vec2::new(TILE_SIZE * GRID_X as f32, TILE_SIZE * GRID_Y as f32)
}
