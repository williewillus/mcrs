use anyhow::Result;
use crate::net::packet::{ClientboundPacket, ServerboundPacket};
use crate::net::proto::{self, ProtoSerializable};
use crate::net::connection::State;
use crate::types::text::Text;
use std::io::{Read, Write};
use uuid::Uuid;

pub struct LoginStart {
    pub name: String
}

impl ServerboundPacket for LoginStart {
    const ID: i32 = 0;
    
    fn state() -> State {
        State::Login
    }

    fn read<R: Read>(mut r: R) -> Result<Self> {
        let name = proto::read::<String, _>(&mut r)?;
        Ok(Self {
            name
        })
    }
}

pub struct Disconnect {
    pub reason: Text,
}

impl ClientboundPacket for Disconnect {
    const ID: i32 = 0;
    
    fn state() -> State {
        State::Login
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let json = serde_json::to_string(&self.reason)?;
        json.write(&mut w)?;
        Ok(())
    }

}

pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
}

impl LoginSuccess {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self {
            uuid,
            username,
        }
    }
}

impl ClientboundPacket for LoginSuccess {
    const ID: i32 = 2;
    
    fn state() -> State {
        State::Login
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        self.uuid.to_string().write(&mut w)?;
        self.username.write(&mut w)?;
        Ok(())
    }
}
