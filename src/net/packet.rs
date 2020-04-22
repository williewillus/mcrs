use anyhow::Result;
use crate::net::connection::State;
use std::io::{Read, Write};

pub trait ServerboundPacket {
    /// The connection state that this packet should be received under
    fn state() -> State;

    /// The packet ID of this packet under the connection state of state()
    fn packet_id() -> i32;

    /// Deserialize the packet from the given stream
    fn read<R: Read>(r: R) -> Result<Self>
    where
        Self: Sized;
}

pub trait ClientboundPacket {
    /// The connection state that this packet should be sent under
    fn state() -> State;

    /// The packet ID of this packet under the connection state of state()
    fn packet_id() -> i32;

    /// Serialize this packet to the given stream
    fn write<W: Write>(&self, w: W) -> Result<()>;
}

pub struct RawPacket {
    pub packet_id: i32,
    pub data: Vec<u8>,
}

