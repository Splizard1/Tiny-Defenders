use bevy_procedural_tilemaps::prelude::*;

pub struct TerrainSockets {
    pub dirt: DirtLayerSockets,
    pub void: Socket,
    pub grass: GrassLayerSockets,
    pub yellow_grass: YellowGrassLayerSockets,
}

pub struct DirtLayerSockets {
    pub layer_up: Socket,   // What can sit on top of dirt
    pub layer_down: Socket, // What dirt can sit on
    pub material: Socket,   // What dirt connects to horizontally
}

pub struct GrassLayerSockets {
    pub layer_up: Socket,
    pub layer_down: Socket,
    pub material: Socket,
    pub void_and_grass: Socket,
    pub grass_and_void: Socket,
    pub grass_fill_up: Socket,
}
pub struct YellowGrassLayerSockets {
    pub layer_up: Socket,
    pub layer_down: Socket,
    pub yellow_grass_fill_down: Socket,
}
pub fn create_sockets(socket_collection: &mut SocketCollection) -> TerrainSockets {
    let mut new_socket = || -> Socket { socket_collection.create() };

    let sockets = TerrainSockets {
        dirt: DirtLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
        },
        void: new_socket(),
        grass: GrassLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            material: new_socket(),
            void_and_grass: new_socket(),
            grass_and_void: new_socket(),
            grass_fill_up: new_socket(),
        },
        yellow_grass: YellowGrassLayerSockets {
            layer_up: new_socket(),
            layer_down: new_socket(),
            yellow_grass_fill_down: new_socket(),
        },
    };
    sockets
}
