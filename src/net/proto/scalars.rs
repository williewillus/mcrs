use anyhow::Result;
use crate::net::proto::ProtoSerializable;
use std::io::{Read, Write};

impl ProtoSerializable for bool {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut buf = [0u8];
        r.read_exact(&mut buf)?;
        Ok(buf[0] == 1)
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let buf = [if *self { 1u8 } else { 0u8 }];
        Ok(w.write_all(&buf)?)
    }
}

impl ProtoSerializable for f32 {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf)?;
        Ok(f32::from_be_bytes(buf))
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let buf = self.to_be_bytes();
        Ok(w.write_all(&buf)?)
    }
}


impl ProtoSerializable for f64 {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(f64::from_be_bytes(buf))
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let buf = self.to_be_bytes();
        Ok(w.write_all(&buf)?)
    }
}

impl ProtoSerializable for u16 {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut buf = [0u8; 2];
        r.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let buf = self.to_be_bytes();
        Ok(w.write_all(&buf)?)
    }
}

impl ProtoSerializable for i64 {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut buf = [0u8; 8];
        r.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let buf = self.to_be_bytes();
        Ok(w.write_all(&buf)?)
    }
}
