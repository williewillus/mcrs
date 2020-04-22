use anyhow::{anyhow, Result};
use crate::net::connection::State;
use crate::net::packet::{ClientboundPacket, ServerboundPacket};
use crate::net::proto::{self, ProtoSerializable};
use std::io::{Read, Write};

struct Request;

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

struct Response {
    /// JSON Encoded status payload
    payload: String,
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

struct Ping(pub i64);

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

struct Pong(i64);

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
