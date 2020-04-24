use anyhow::{anyhow, Result};
use std::io::Read;
use crate::net::connection::State;
use crate::net::packet::ServerboundPacket;
use crate::net::proto;

#[derive(Debug)]
pub struct Handshake {
    pub proto_version: i32,
    pub addr: String,
    pub port: u16,
    pub next_state: State,
}

impl ServerboundPacket for Handshake {
    const ID: i32 = 0;
    
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let proto_version = proto::read_varint(&mut r)?;
        let addr = proto::read(&mut r)?;
        let port = proto::read(&mut r)?;
        let next = proto::read_varint(&mut r)?;
        let next_state = match next {
            1 => State::Status,
            2 => State::Login,
            _ => return Err(anyhow!("Unknown next state {}", next)),
        };
        
        Ok(Self {
            proto_version,
            addr,
            port,
            next_state,
        })
    }
}
