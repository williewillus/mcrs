use anyhow::{anyhow, Result};
use crossbeam::crossbeam_channel::{Receiver, Sender};
use log::{debug, warn, trace};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use crate::net::{handshake, login, play, proto, status};
use crate::net::packet::{ClientboundPacket, ServerboundPacket, RawPacket};
use crate::server::Server;
use crate::types::text::Text;

#[derive(Debug, PartialEq, Eq)]
pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Connection {
    state: State,
    stream: TcpStream,
    server: Arc<Server>,

}

/// The maximum declared size of a packet. Currently set to 2MiB to match vanilla
const MAX_PACKET_BYTES: usize = 2 * 1024 * 1024;

impl Connection {
    pub fn new(stream: TcpStream, server: Arc<Server>) -> Self {
        Self {
            state: State::Handshake,
            stream,
            server,
        }
    }

    fn read_raw_packet(&mut self) -> Result<RawPacket> {
        let len = proto::read_varint(&mut self.stream)?;
        if len < 0 || len as usize > MAX_PACKET_BYTES {
            return Err(anyhow!("Packet size invalid"));
        }
        let mut buf = vec![0; len as usize];
        self.stream.read_exact(&mut buf)?;

        let mut cursor = buf.as_slice();
        // Read the packet ID out, which updates ``cursor`` to point at the data
        let packet_id = proto::read_varint(&mut cursor)?;

        // Read the data itself. TODO: Try to eliminate extra allocation here? Can we just truncate then pass ownership of buf to rawpacket?
        let data = cursor.iter().copied().collect();

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

    fn send_packet<T: ClientboundPacket>(&mut self, packet: &T) -> Result<()> {
        let mut buf = Vec::new();
        let id = T::packet_id();
        proto::write_varint(&mut buf, id)?;
        packet.write(&mut buf)?;

        let len = buf.len() as i32;
        proto::write_varint(&mut self.stream, len)?;
        Ok(self.stream.write_all(buf.as_slice())?)
    }

    fn process_handshake(&mut self) -> Result<()> {
        let buf = &mut [0u8];
        self.stream.peek(buf)?;
        if buf[0] == 0xFE {
            return Err(anyhow!("Disconnecting: currently not handling legacy server ping"));
        }
        let handshake = self.read_expected_packet::<handshake::Handshake>()?;
        if handshake.proto_version != proto::PROTO_VERSION {
            return Err(anyhow!("Only supports protocol {}", proto::PROTO_VERSION));
        }
        match handshake.next_state {
            State::Status => self.state = State::Status,
            State::Login => self.state = State::Login,
            _ => return Err(anyhow!("Invalid handshake state transition")),
        }

        debug!("Switched to state {:?}", self.state);
        Ok(())
    }


    /// Read and process one Status state packet. Returns true when the server should disconnect.
    fn process_status(&mut self) -> Result<bool> {
        let raw = self.read_raw_packet()?;
        let mut buf = raw.data.as_slice();

        let should_disconnect = match raw.packet_id {
            0 => {
                trace!("Status request");
                let _req = status::Request::read(&mut buf);

                let version = status::VersionStatus::new(proto::PROTO_NAME.to_string(), proto::PROTO_VERSION);
                let players = status::PlayerStatus::new(25, 0, Vec::new());
                let description = Text { text: "mcrs server".to_string() };
                let payload = status::StatusPayload::new(version, players, description, None);
                let resp = status::Response::new(&payload);

                self.send_packet(&resp)?;
                false
            },
            1 => {
                trace!("Status ping");
                let ping = status::Ping::read(&mut buf)?;
                let resp = status::Pong(ping.0);
                self.send_packet(&resp)?;
                true
            },
            _ => return Err(anyhow!("Unknown status packet {}", raw.packet_id)),
        };

        // read will update `buf` to point to the unread part, so we know if the packet didn't read everything
        if !buf.is_empty() {
            warn!("Status packet did not drain its payload");
        }
        trace!("Status success");
        Ok(should_disconnect)
    }

    fn process_login(&mut self) -> Result<()> {
        let login_start = self.read_expected_packet::<login::LoginStart>()?;
        trace!("login start from username {}", login_start.name);
        let uuid = uuid::Uuid::new_v4();
        let resp = login::LoginSuccess::new(uuid, login_start.name);
        self.send_packet(&resp)?;
        self.state = State::Play;
        Ok(())
    }

    /// Process packets during ordinary play. At this time, the main server thread has taken over all logic duties,
    /// and this loop should simply be draining the send queue and forwarding messages to the server message queue
    fn process_play(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn process(&mut self) -> Result<()> {
        loop {
            match self.state {
                State::Handshake => self.process_handshake()?,
                State::Status => {
                    let should_disconnect = self.process_status()?;
                    if should_disconnect {
                        return Ok(())
                    }
                },
                State::Login => self.process_login()?,
                State::Play => self.process_play()?,
            }
        }
    }
}
