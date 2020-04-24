use crate::net::play::{ClientboundPlayPacket, ServerboundPlayPacket};
use crossbeam::crossbeam_channel::{Receiver, Sender};
use std::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

struct PlayerConnection {
    /// Channel of inbound play packets from this player
    inbound: Receiver<ServerboundPlayPacket>,

    /// Channel of outbound play packets for this player
    outbound: Sender<ClientboundPlayPacket>,
}

pub struct Server {
    player_connections: RwLock<HashMap<Uuid, PlayerConnection>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            player_connections: RwLock::new(HashMap::new()),
        }
    }

    pub fn add_player(&self, uuid: Uuid, inbound: Receiver<ServerboundPlayPacket>, outbound: Sender<ClientboundPlayPacket>) {
        let conn = PlayerConnection {
            inbound,
            outbound,
        };
        self.player_connections.write().unwrap().insert(uuid, conn);
    }
}
