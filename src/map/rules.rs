use crate::map::assets::SpawnableAsset;
use crate::map::models::TerrainModelBuilder;
use crate::map::sockets::*;
use bevy_procedural_tilemaps::prelude::*;

fn build_dirt_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection,
) {
    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.dirt.material,   // right
                x_neg: terrain_sockets.dirt.material,   // left
                z_pos: terrain_sockets.dirt.layer_up,   // top
                z_neg: terrain_sockets.dirt.layer_down, // bottom
                y_pos: terrain_sockets.dirt.material,   // up
                y_neg: terrain_sockets.dirt.material,   // down
            },
            vec![SpawnableAsset::new("dirt")],
        )
        .with_weight(20.);

    socket_collection.add_connections(vec![(
        terrain_sockets.dirt.material,
        vec![terrain_sockets.dirt.material],
    )]);
}

pub fn build_world() -> (
    Vec<Vec<SpawnableAsset>>,
    ModelCollection<Cartesian3D>,
    SocketCollection,
) {
    let mut socket_collection = SocketCollection::new();
    let terrain_sockets = create_sockets(&mut socket_collection);

    let mut terrain_model_builder = TerrainModelBuilder::new();

    // Build dirt layer
    build_dirt_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection,
    );

    let (assets, models) = terrain_model_builder.into_parts();

    (assets, models, socket_collection)
}
