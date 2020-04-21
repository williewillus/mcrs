use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use crate::net::connection::State;
use crate::net::packet::Packet;
use crate::net::proto;

struct Handshake {
    proto_version: i32,
    addr: String,
    port: u16,
    next_state: State,
}

impl Packet for Handshake {
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

    fn write<W: Write>(&self, _: W) -> Result<()> {
        unimplemented!("Handshake packet only needs to be readable")
    }
}
