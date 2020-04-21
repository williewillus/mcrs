use anyhow::Result;
use std::io::{Read, Write};

pub trait Packet {
    fn read<R: Read>(r: R) -> Result<Self>
    where
        Self: Sized;

    fn write<W: Write>(&self, w: W) -> Result<()>;
}
