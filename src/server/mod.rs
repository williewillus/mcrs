use crossbeam::crossbeam_channel::{Receiver, Sender};
use std::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;

struct PlayerConnection {
    /// Channel of inbound play packets from this player
    inbound: Receiver<()>,

    /// Channel of outbound play packets for this player
    outbound: Sender<()>,
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

    pub fn add_player(&self, uuid: Uuid, inbound: Receiver<()>, outbound: Sender<()>) {
        let conn = PlayerConnection {
            inbound,
            outbound,
        };
        self.player_connections.write().unwrap().insert(uuid, conn);
    }
}
