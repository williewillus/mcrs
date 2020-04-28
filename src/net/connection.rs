use anyhow::{anyhow, Result};
use crossbeam::channel::Sender;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
use crate::net::{handshake, login, play, proto, status};
use crate::net::packet::RawPacket;
use crate::server::Server;
use crate::types::text::Text;

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
    play_state: Option<PlayState>,
}

/// The maximum declared size of a packet. Currently set to 2MiB to match vanilla
const MAX_PACKET_BYTES: usize = 2 * 1024 * 1024;

impl Connection {
    pub fn new(stream: TcpStream, server: Arc<Server>) -> Self {
        Self {
            state: State::Handshake,
            stream,
            server,
            play_state: None,
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

    fn process_handshake(&mut self) -> Result<()> {
        let buf = &mut [0u8];
        self.stream.peek(buf)?;
        if buf[0] == 0xFE {
            return Err(anyhow!("Disconnecting: currently not handling legacy server ping"));
        }
        let raw = self.read_raw_packet()?;
        if raw.packet_id != 0 {
            return Err(anyhow!("Unknown handshake packet id {}", raw.packet_id));
        }

        let handshake = handshake::Handshake::read(&mut raw.data.as_slice())?;
        if handshake.proto_version != proto::PROTO_VERSION {
            return Err(anyhow!("Only supports protocol {}", proto::PROTO_VERSION));
        }
        match handshake.next_state {
            State::Status => {
                log::trace!("Switched to status state");
                self.state = State::Status;
            },
            State::Login => {
                log::trace!("Switched to login state");
                self.state = State::Login;
            }
            _ => return Err(anyhow!("Invalid handshake state transition")),
        }

        Ok(())
    }


    /// Read and process one Status state packet. Returns true when the server should disconnect.
    fn process_status(&mut self) -> Result<bool> {
        use status::serverbound::Packet as SPacket;
        use status::clientbound::{Packet as CPacket, Pong, Response};

        let raw = self.read_raw_packet()?;
        let pkt = raw.try_into()?;

        let should_disconnect = match pkt {
            SPacket::Request(payload) => {
                log::trace!("Status request");
                let version = status::VersionStatus::new(proto::PROTO_NAME.to_string(), proto::PROTO_VERSION);
                let players = status::PlayerStatus::new(25, 0, Vec::new());
                let description = Text { text: "mcrs server".to_string() };
                let status = status::StatusPayload::new(version, players, description, None);
                let resp = CPacket::Response(Response { status });
                resp.write(&mut self.stream)?;
                false
            },
            SPacket::Ping(payload) => {
                log::trace!("Status ping");
                let resp = CPacket::Pong(Pong { val: payload.val });
                resp.write(&mut self.stream)?;
                true
            },
        };

        log::trace!("Status success");
        Ok(should_disconnect)
    }

    fn process_login(&mut self) -> Result<()> {
        use login::serverbound::Packet as SPacket;
        use login::clientbound::{Packet as CPacket, LoginSuccess};

        let raw_pkt = self.read_raw_packet()?;
        let pkt = raw_pkt.try_into()?;
        match pkt {
            SPacket::LoginStart(payload) => {
                log::trace!("login start from username {}", payload.name);
                let thread_name = format!("outbound network to {}", payload.name);

                let uuid = uuid::Uuid::new_v4();
                let resp = CPacket::LoginSuccess(LoginSuccess { uuid, name: payload.name });
                resp.write(&mut self.stream)?;

                let (inbound_sender, inbound_receiver) = crossbeam::channel::unbounded();
                let (outbound_sender, outbound_receiver) = crossbeam::channel::unbounded();

                self.server.add_player(uuid, inbound_receiver, outbound_sender);

                let mut socket_clone = self.stream.try_clone()?;
                let outbound_handle = thread::Builder::new()
                    .name(thread_name)
                    .spawn(move || {
                        for item in outbound_receiver {
                            if let Err(e) = item.write(&mut socket_clone) {
                                log::error!("Outbound thread errored: {}", e);
                                break;
                            }
                        }
                    })?;

                self.play_state = Some(PlayState {
                    outbound: outbound_handle,
                    inbound: inbound_sender,
                });
                log::trace!("switched to play state");
                self.state = State::Play;
                Ok(())
            },
        }
        
    }

    /// Process packets during ordinary play. At this time, the main server thread has taken over all logic duties.
    /// A separate thread has been spawned to flush the outbound queue. The sole responsibility of this method now is to read inbound messages
    /// and hand them to the server.
    fn process_play(&mut self) -> Result<()> {
        let raw = self.read_raw_packet()?;
        let play_state = self.play_state.as_ref().unwrap();
        match raw.try_into() {
            Ok(pkt) => play_state.inbound.send(pkt)?,
            Err(e) => log::error!("Error parsing play packet, ignoring: {}", e),
        }
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

pub struct PlayState {
    /// Handle to the outbound thread
    outbound: thread::JoinHandle<()>,

    /// Queue to transfer packets to the server thread
    inbound: Sender<play::serverbound::Packet>,
}
