use anyhow::Result;
use std::io::{Read, Write};

pub trait ServerboundPacket {
    /// The packet ID of this packet under the current connection state
    const ID: i32;

    /// Deserialize the packet from the given stream
    fn read<R: Read>(r: R) -> Result<Self>
    where
        Self: Sized;
}

pub trait ClientboundPacket {
    /// The packet ID of this packet under the current connection state
    const ID: i32;
    
    /// Serialize this packet to the given stream
    fn write<W: Write>(&self, w: W) -> Result<()>;
}

pub struct RawPacket {
    pub packet_id: i32,
    pub data: Vec<u8>,
}

