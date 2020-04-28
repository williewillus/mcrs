//! This module and its children contain `ProtoSerializable`, representing anything that can be written
//! and read from the network in a general manner, as well as impls for common types.
mod scalars;
mod string;
pub mod varint;

use anyhow::Result;
use std::io::{Read, Write};

pub const PROTO_NAME: &str = "1.15.2";
pub const PROTO_VERSION: i32 = 578;

pub fn read<T, R>(mut r: R) -> Result<T>
where T: ProtoSerializable,
      R: Read
{
    <T as ProtoSerializable>::read(&mut r)
}

/// Helper function to read a varint from a stream without going through the wrapper type
pub fn read_varint<R: Read>(mut r: R) -> Result<i32> {
    let v = read::<varint::Varint, _>(&mut r)?;
    Ok(v.0)
}

/// Helper function to write a varint into a stream without going through the wrapper type
pub fn write_varint<W: Write>(mut w: W, v: i32) -> Result<()> {
    let v = varint::Varint(v);
    v.write(&mut w)
}

pub trait ProtoSerializable {
    fn read<R: Read>(r: R) -> Result<Self>
        where Self: Sized;
    fn write<W: Write>(&self, w: W) -> Result<()>;
}
