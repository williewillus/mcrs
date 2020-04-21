use anyhow::{anyhow, Result};
use std::net::TcpStream;
use crate::net::handshake::Handshake;
use crate::net::packet::Packet;

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Connection {
    state: State,
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            state: State::Handshake,
            stream,
        }
    }

    fn read_packet() {
    }

    pub fn process(&mut self) -> Result<()> {
        // first packet
        let handshake = Handshake::read(&mut self.stream)?;
        if handshake.proto_version != 578 {
            return Err(anyhow!("Only supports 1.15.2 protocol 578"));
        }
        Ok(())
    }
}
