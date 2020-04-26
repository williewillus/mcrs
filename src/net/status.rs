use crate::types::text::Text;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
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

pub mod serverbound {
    packets! {
        0 => Request {},
        1 => Ping { val: i64 }
    }
}

pub mod clientbound {
    use super::StatusPayload;
    
    packets! {
        0 => Response { status: StatusPayload;
                        impl ProtoSerializable for Response {
                            fn read<R: Read>(mut r: R) -> Result<Self>
                                where Self: Sized
                            {
                                let s: String = proto::read(&mut r)?;
                                let status = serde_json::from_str(&s)?;
                                Ok(Self {
                                    status
                                })
                            }

                            fn write<W: Write>(&self, mut w: W) -> Result<()> {
                                let s = serde_json::to_string(&self.status)?;
                                s.write(&mut w)
                            }
                        }
                        
        },
        1 => Pong { val: i64 }
    }
}
