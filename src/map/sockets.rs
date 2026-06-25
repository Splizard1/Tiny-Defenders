use bevy_procedural_tilemaps::prelude::*;

pub struct TerrainSockets {
    pub dirt: DirtLayerSockets,
}

pub struct DirtLayerSockets {
    pub layer_up: Socket,   // What can sit on top of dirt
    pub layer_down: Socket, // What dirt can sit on
    pub material: Socket,   // What dirt connects to horizontally
}

pub fn create_sockets(socket_collection: &mut SocketCollection) -> TerrainSockets {
    let mut new_socket = || -> Socket { socket_collection.create() };

    let sockets = TerrainSockets {
        dirt: DirtLayerSockets {
            layer_up: new_socket(),
            material: new_socket(),
            layer_down: new_socket(),
        },
    };
    sockets
}
