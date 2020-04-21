mod ints;
mod string;
mod varint;

use std::io::{Read, Write};

pub fn read<T, R>(mut r: R) -> anyhow::Result<T>
where T: ProtoSerializable,
      R: Read
{
    <T as ProtoSerializable>::read(&mut r)
}

pub fn read_varint<R: Read>(mut r: R) -> anyhow::Result<i32> {
    let v = read::<varint::Varint, _>(&mut r)?;
    Ok(v.0)
}

pub trait ProtoSerializable {
    fn read<R: Read>(r: R) -> anyhow::Result<Self>
        where Self: Sized;
    fn write<W: Write>(&self, w: W) -> anyhow::Result<()>;
}
