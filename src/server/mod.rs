use anyhow::{anyhow, Result};
use crate::net::play::{self, clientbound::Packet as CPacket, serverbound::Packet as SPacket};
use crossbeam::crossbeam_channel::{Receiver, Sender};
use std::convert::TryInto;
use std::sync::Mutex;
use std::collections::HashMap;
use uuid::Uuid;

const TARGET_MSPT: usize = 50;

struct PlayerConnection {
    ticks: u32,
    
    /// Channel of inbound play packets from this player
    inbound: Receiver<play::serverbound::Packet>,

    /// Channel of outbound play packets for this player
    outbound: Sender<play::clientbound::Packet>,
}

pub struct Server {
    player_connections: Mutex<HashMap<Uuid, PlayerConnection>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            player_connections: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_player(&self, uuid: Uuid, inbound: Receiver<play::serverbound::Packet>, outbound: Sender<play::clientbound::Packet>) {
        let conn = PlayerConnection {
            ticks: 0,
            inbound,
            outbound,
        };
        self.player_connections.lock().unwrap().insert(uuid, conn);
    }

    fn tick(&self) -> Result<bool> {
        for (uuid, conn) in self.player_connections.lock().unwrap().iter_mut() {
            if conn.ticks == 0 {
                log::trace!("First play tick for {}", uuid);
                let msg = play::clientbound::JoinGame {
                    entity_id: 0,
                    gamemode: 1,
                    dimension: 0,
                    hashed_seed: 0,
                    max_players: 0,
                    level_type: "default".to_owned(),
                    view_dist: 8.into(),
                    reduced_debug: false,
                    respawn_screen: true,
                    
                };
                conn.outbound.send(msg.into())?;
            }
            conn.ticks += 1;
        }
        Ok(false)
    }

    pub fn run(&self) -> Result<()> {
        loop {
            let start = std::time::Instant::now();
            if self.tick()? {
                break;
            }
            let end = std::time::Instant::now() - start;
            if end.as_millis() < 50 {
                let slp_ms = 50 - end.as_millis();
                let slp_dur = std::time::Duration::from_millis(slp_ms.try_into().unwrap());
                std::thread::sleep(slp_dur);
            }
        }
        Ok(())
    }
}
