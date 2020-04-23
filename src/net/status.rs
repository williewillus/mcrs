use anyhow::Result;
use crate::net::connection::State;
use crate::net::packet::{ClientboundPacket, ServerboundPacket};
use crate::net::proto::{self, ProtoSerializable};
use crate::types::text::Text;
use serde::Serialize;
use std::io::{Read, Write};

pub struct Request;

impl ServerboundPacket for Request {
    fn state() -> State {
        State::Status
    }

    fn packet_id() -> i32 {
        0
    }

    fn read<R: Read>(_: R) -> Result<Self> {
        Ok(Self)
    }
}

#[derive(Serialize)]
pub struct VersionStatus {
    /// String name of the current version, e.g. "1.15.2"
    name: String,
    /// Protocol number of the current version
    protocol: i32,
}

impl VersionStatus {
    pub fn new(name: impl ToString, protocol: i32) -> Self {
        Self {
            name: name.to_string(),
            protocol,
        }
    }
}

#[derive(Serialize)]
pub struct PlayerSample {
    name: String,
    /// Uuid of the player
    id: String,
}

impl PlayerSample {
    pub fn new(name: impl ToString, id: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            id: id.to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct PlayerStatus {
    max: u32,
    online: u32,
    sample: Vec<PlayerSample>,
}

impl PlayerStatus {
    pub fn new(max: u32, online: u32, sample: Vec<PlayerSample>) -> Self {
        Self {
            max,
            online,
            sample,
        }
    }
}

#[derive(Serialize)]
pub struct StatusPayload {
    version: VersionStatus,
    players: PlayerStatus,
    description: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>,
}

impl StatusPayload {
    pub fn new(version: VersionStatus, players: PlayerStatus, description: Text, favicon: Option<String>) -> Self {
        Self {
            version,
            players,
            description,
            favicon,
        }
    }
}

pub struct Response {
    /// JSON Encoded status payload
    payload: String,
}

impl Response {
    pub fn new(status: &StatusPayload) -> Self {
        let payload = serde_json::to_string(status).unwrap(); // FIXME: This unwrap should be okay, but recheck
        Self {
            payload
        }
    }
}

impl ClientboundPacket for Response {
    fn state() -> State {
        State::Status
    }

    fn packet_id() -> i32 {
        0
    }
    
    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        self.payload.write(&mut w)
    }
}

pub struct Ping(pub i64);

impl ServerboundPacket for Ping {
    fn state() -> State {
        State::Status
    }

    fn packet_id() -> i32 {
        1
    }
    
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let v = proto::read::<i64, _>(&mut r)?;
        Ok(Ping(v))
    }

}

pub struct Pong(pub i64);

impl ClientboundPacket for Pong {
    fn state() -> State {
        State::Status
    }

    fn packet_id() -> i32 {
        1
    }
    
    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        self.0.write(&mut w)
    }
}
