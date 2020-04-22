use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use std::net::TcpStream;
use crate::net::handshake::Handshake;
use crate::net::proto;
use crate::net::packet::{ClientboundPacket, ServerboundPacket, RawPacket};

#[derive(PartialEq, Eq)]
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

/// The maximum declared size of a packet. Currently set to 2MiB to match vanilla
const MAX_PACKET_BYTES: usize = 2 * 1024 * 1024;

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            state: State::Handshake,
            stream,
        }
    }

    fn read_raw_packet(&mut self) -> Result<RawPacket> {
        let len = proto::read_varint(&mut self.stream)?;
        if len < 0 || len as usize > MAX_PACKET_BYTES {
            return Err(anyhow!("Packet size invalid"));
        }
        let mut buf = Vec::with_capacity(len as usize);
        self.stream.read_exact(&mut buf)?;

        let mut cursor = buf.as_slice();
        // Read the packet ID out, which updates ``cursor`` to point at the data
        let packet_id = proto::read_varint(&mut cursor)?;

        // Read the data itself. TODO: Try to eliminate extra allocation here? Can we just truncate then pass ownership of buf to rawpacket?
        let data = buf.iter().copied().collect();

        Ok(RawPacket {
            packet_id,
            data,
        })
    }

    fn read_expected_packet<T: ServerboundPacket>(&mut self) -> Result<T> {
        if self.state != T::state() {
            return Err(anyhow!("Invalid state to receive packet {}", std::any::type_name::<T>()));
        }
        
        let raw = self.read_raw_packet()?;
        if raw.packet_id != T::packet_id() {
            return Err(anyhow!("Expected packet id {} but it was {}", T::packet_id(), raw.packet_id));
        }

        Ok(T::read(raw.data.as_slice())?)
    }

    fn send_packet<T: ClientboundPacket>(&mut self, packet: T) -> Result<()> {
        let mut buf = Vec::new();
        let id = T::packet_id();
        proto::write_varint(&mut buf, id)?;
        packet.write(&mut buf)?;

        let len = buf.len() as i32;
        proto::write_varint(&mut self.stream, len)?;
        Ok(self.stream.write_all(buf.as_slice())?)
    }

    fn process_handshake(&mut self) -> Result<()> {
        let handshake = self.read_expected_packet::<Handshake>()?;
        if handshake.proto_version != 578 {
            return Err(anyhow!("Only supports 1.15.2 protocol 578"));
        }
        match handshake.next_state {
            State::Status => self.state = State::Status,
            State::Login => unimplemented!(),
            _ => return Err(anyhow!("Invalid handshake state transition")),
        }

        Ok(())
    }


    fn process_status(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn process(&mut self) -> Result<()> {
        loop {
            match self.state {
                State::Handshake => self.process_handshake()?,
                State::Status => self.process_status()?,
                State::Login => unimplemented!(),
                State::Play => unimplemented!(),
            }
        }
    }
}
