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
    // Build grass layer
    build_grass_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection,
    );
    // Build yellow grass layer
    build_yellow_grass_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection,
    );

    build_water_layer(
        &mut terrain_model_builder,
        &terrain_sockets,
        &mut socket_collection,
    );

    let (assets, models) = terrain_model_builder.into_parts();

    (assets, models, socket_collection)
}
fn build_grass_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection,
) {
    // Void model - empty space above dirt where no grass exists
    terrain_model_builder.create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.void,
            x_neg: terrain_sockets.void,
            z_pos: terrain_sockets.grass.layer_up,
            z_neg: terrain_sockets.grass.layer_down,
            y_pos: terrain_sockets.void,
            y_neg: terrain_sockets.void,
        },
        Vec::new(),
    );

    // Main grass tile
    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Multiple {
                x_pos: vec![terrain_sockets.grass.material],
                x_neg: vec![terrain_sockets.grass.material],
                z_pos: vec![
                    terrain_sockets.grass.layer_up,
                    terrain_sockets.grass.grass_fill_up,
                ],
                z_neg: vec![terrain_sockets.grass.layer_down],
                y_pos: vec![terrain_sockets.grass.material],
                y_neg: vec![terrain_sockets.grass.material],
            },
            vec![SpawnableAsset::new("green_grass")],
        )
        .with_weight(5.);

    // Outer corner template
    let green_grass_corner_out = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.void,
        z_pos: terrain_sockets.grass.layer_up,
        z_neg: terrain_sockets.grass.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.grass_and_void,
    }
    .to_template();

    // Inner corner template
    let green_grass_corner_in = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.grass_and_void,
        x_neg: terrain_sockets.grass.material,
        z_pos: terrain_sockets.grass.layer_up,
        z_neg: terrain_sockets.grass.layer_down,
        y_pos: terrain_sockets.grass.material,
        y_neg: terrain_sockets.grass.void_and_grass,
    }
    .to_template();

    // Side edge template
    let green_grass_side = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.grass.grass_and_void,
        z_pos: terrain_sockets.grass.layer_up,
        z_neg: terrain_sockets.grass.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.material,
    }
    .to_template();

    // Create rotated versions of outer corners
    terrain_model_builder.create_model(
        green_grass_corner_out.clone(),
        vec![SpawnableAsset::new("green_grass_corner_out_tl")],
    );
    terrain_model_builder.create_model(
        green_grass_corner_out.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_corner_out_bl")],
    );
    terrain_model_builder.create_model(
        green_grass_corner_out.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_corner_out_br")],
    );
    terrain_model_builder.create_model(
        green_grass_corner_out.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_corner_out_tr")],
    );

    // Create rotated versions of inner corners
    terrain_model_builder.create_model(
        green_grass_corner_in.clone(),
        vec![SpawnableAsset::new("green_grass_corner_in_tl")],
    );
    terrain_model_builder.create_model(
        green_grass_corner_in.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_corner_in_bl")],
    );
    terrain_model_builder.create_model(
        green_grass_corner_in.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_corner_in_br")],
    );
    terrain_model_builder.create_model(
        green_grass_corner_in.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_corner_in_tr")],
    );

    // Create rotated versions of side edges
    terrain_model_builder.create_model(
        green_grass_side.clone(),
        vec![SpawnableAsset::new("green_grass_side_t")],
    );
    terrain_model_builder.create_model(
        green_grass_side.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_side_l")],
    );
    terrain_model_builder.create_model(
        green_grass_side.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_side_b")],
    );
    terrain_model_builder.create_model(
        green_grass_side.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("green_grass_side_r")],
    );

    // Add connection rules
    socket_collection.add_rotated_connection(
        terrain_sockets.dirt.layer_up,
        vec![terrain_sockets.grass.layer_down],
    );
    socket_collection.add_connections(vec![
        (terrain_sockets.void, vec![terrain_sockets.void]),
        (
            terrain_sockets.grass.material,
            vec![terrain_sockets.grass.material],
        ),
        (
            terrain_sockets.grass.void_and_grass,
            vec![terrain_sockets.grass.grass_and_void],
        ),
    ]);
}
fn build_yellow_grass_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection,
) {
    // Void model - empty space where no yellow grass exists
    terrain_model_builder.create_model(
        SocketsCartesian3D::Simple {
            x_pos: terrain_sockets.void,
            x_neg: terrain_sockets.void,
            z_pos: terrain_sockets.yellow_grass.layer_up,
            z_neg: terrain_sockets.yellow_grass.layer_down,
            y_pos: terrain_sockets.void,
            y_neg: terrain_sockets.void,
        },
        Vec::new(),
    );

    // Main yellow grass tile
    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.grass.material,
                x_neg: terrain_sockets.grass.material,
                z_pos: terrain_sockets.yellow_grass.layer_up,
                z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
                y_pos: terrain_sockets.grass.material,
                y_neg: terrain_sockets.grass.material,
            },
            vec![SpawnableAsset::new("yellow_grass")],
        )
        .with_weight(5.);

    // Outer corner template
    let yellow_grass_corner_out = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.void,
        z_pos: terrain_sockets.yellow_grass.layer_up,
        z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.grass_and_void,
    }
    .to_template();

    // Inner corner template
    let yellow_grass_corner_in = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.grass_and_void,
        x_neg: terrain_sockets.grass.material,
        z_pos: terrain_sockets.yellow_grass.layer_up,
        z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
        y_pos: terrain_sockets.grass.material,
        y_neg: terrain_sockets.grass.void_and_grass,
    }
    .to_template();

    // Side edge template
    let yellow_grass_side = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.grass.void_and_grass,
        x_neg: terrain_sockets.grass.grass_and_void,
        z_pos: terrain_sockets.yellow_grass.layer_up,
        z_neg: terrain_sockets.yellow_grass.yellow_grass_fill_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.grass.material,
    }
    .to_template();

    // Create rotated versions of outer corners
    terrain_model_builder.create_model(
        yellow_grass_corner_out.clone(),
        vec![SpawnableAsset::new("yellow_grass_corner_out_tl")],
    );
    terrain_model_builder.create_model(
        yellow_grass_corner_out.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_corner_out_bl")],
    );
    terrain_model_builder.create_model(
        yellow_grass_corner_out.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_corner_out_br")],
    );
    terrain_model_builder.create_model(
        yellow_grass_corner_out.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_corner_out_tr")],
    );

    // Create rotated versions of inner corners
    terrain_model_builder.create_model(
        yellow_grass_corner_in.clone(),
        vec![SpawnableAsset::new("yellow_grass_corner_in_tl")],
    );
    terrain_model_builder.create_model(
        yellow_grass_corner_in.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_corner_in_bl")],
    );
    terrain_model_builder.create_model(
        yellow_grass_corner_in.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_corner_in_br")],
    );
    terrain_model_builder.create_model(
        yellow_grass_corner_in.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_corner_in_tr")],
    );

    // Create rotated versions of side edges
    terrain_model_builder.create_model(
        yellow_grass_side.clone(),
        vec![SpawnableAsset::new("yellow_grass_side_t")],
    );
    terrain_model_builder.create_model(
        yellow_grass_side.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_side_l")],
    );
    terrain_model_builder.create_model(
        yellow_grass_side.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_side_b")],
    );
    terrain_model_builder.create_model(
        yellow_grass_side.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("yellow_grass_side_r")],
    );

    // Add connection rules
    socket_collection
        .add_rotated_connection(
            terrain_sockets.grass.layer_up,
            vec![terrain_sockets.yellow_grass.layer_down],
        )
        .add_rotated_connection(
            terrain_sockets.yellow_grass.yellow_grass_fill_down,
            vec![terrain_sockets.grass.grass_fill_up],
        );
}

pub fn build_water_layer(
    terrain_model_builder: &mut TerrainModelBuilder,
    terrain_sockets: &TerrainSockets,
    socket_collection: &mut SocketCollection,
) {
    // Void model - represents land areas where no water exists
    terrain_model_builder.create_model(
        SocketsCartesian3D::Multiple {
            x_pos: vec![terrain_sockets.void],
            x_neg: vec![terrain_sockets.void],
            z_pos: vec![
                terrain_sockets.water.layer_up,
                terrain_sockets.water.ground_up,
            ],
            z_neg: vec![terrain_sockets.water.layer_down],
            y_pos: vec![terrain_sockets.void],
            y_neg: vec![terrain_sockets.void],
        },
        Vec::new(),
    );

    // Main water tile
    const WATER_WEIGHT: f32 = 0.02;
    terrain_model_builder
        .create_model(
            SocketsCartesian3D::Simple {
                x_pos: terrain_sockets.water.material,
                x_neg: terrain_sockets.water.material,
                z_pos: terrain_sockets.water.layer_up,
                z_neg: terrain_sockets.water.layer_down,
                y_pos: terrain_sockets.water.material,
                y_neg: terrain_sockets.water.material,
            },
            vec![SpawnableAsset::new("water")],
        )
        .with_weight(10. * WATER_WEIGHT);

    // Outer corner template
    let water_corner_out = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.water.void_and_water,
        x_neg: terrain_sockets.void,
        z_pos: terrain_sockets.water.layer_up,
        z_neg: terrain_sockets.water.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.water.water_and_void,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    // Inner corner template
    let water_corner_in = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.water.water_and_void,
        x_neg: terrain_sockets.water.material,
        z_pos: terrain_sockets.water.layer_up,
        z_neg: terrain_sockets.water.layer_down,
        y_pos: terrain_sockets.water.material,
        y_neg: terrain_sockets.water.void_and_water,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    // Side edge template
    let water_side = SocketsCartesian3D::Simple {
        x_pos: terrain_sockets.water.void_and_water,
        x_neg: terrain_sockets.water.water_and_void,
        z_pos: terrain_sockets.water.layer_up,
        z_neg: terrain_sockets.water.layer_down,
        y_pos: terrain_sockets.void,
        y_neg: terrain_sockets.water.material,
    }
    .to_template()
    .with_weight(WATER_WEIGHT);

    // Create rotated versions of outer corners
    terrain_model_builder.create_model(
        water_corner_out.clone(),
        vec![SpawnableAsset::new("water_corner_out_tl")],
    );
    terrain_model_builder.create_model(
        water_corner_out.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("water_corner_out_bl")],
    );
    terrain_model_builder.create_model(
        water_corner_out.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("water_corner_out_br")],
    );
    terrain_model_builder.create_model(
        water_corner_out.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("water_corner_out_tr")],
    );

    // Create rotated versions of inner corners
    terrain_model_builder.create_model(
        water_corner_in.clone(),
        vec![SpawnableAsset::new("water_corner_in_tl")],
    );
    terrain_model_builder.create_model(
        water_corner_in.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("water_corner_in_bl")],
    );
    terrain_model_builder.create_model(
        water_corner_in.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("water_corner_in_br")],
    );
    terrain_model_builder.create_model(
        water_corner_in.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("water_corner_in_tr")],
    );

    // Create rotated versions of side edges
    terrain_model_builder.create_model(
        water_side.clone(),
        vec![SpawnableAsset::new("water_side_t")],
    );
    terrain_model_builder.create_model(
        water_side.rotated(ModelRotation::Rot90, Direction::ZForward),
        vec![SpawnableAsset::new("water_side_l")],
    );
    terrain_model_builder.create_model(
        water_side.rotated(ModelRotation::Rot180, Direction::ZForward),
        vec![SpawnableAsset::new("water_side_b")],
    );
    terrain_model_builder.create_model(
        water_side.rotated(ModelRotation::Rot270, Direction::ZForward),
        vec![SpawnableAsset::new("water_side_r")],
    );

    // Add connection rules
    socket_collection.add_connections(vec![
        (
            terrain_sockets.water.material,
            vec![terrain_sockets.water.material],
        ),
        (
            terrain_sockets.water.water_and_void,
            vec![terrain_sockets.water.void_and_water],
        ),
    ]);

    // Connect water layer to yellow grass layer
    socket_collection.add_rotated_connection(
        terrain_sockets.yellow_grass.layer_up,
        vec![terrain_sockets.water.layer_down],
    );
}
