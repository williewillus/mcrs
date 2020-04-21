use anyhow::{anyhow, Result};
use crate::net::proto::{ProtoSerializable, self, varint::Varint};
use std::io::{Read, Write};

const LENGTH_LIMIT: usize = 32767;

impl ProtoSerializable for String {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let len = proto::read::<Varint, _>(&mut r)?.0;
        if len < 0 || len as usize > LENGTH_LIMIT {
            Err(anyhow!("Invalid length {}", len))
        } else {
            let mut buf = vec![0; len as usize];
            r.read_exact(&mut buf)?;
            Ok(String::from_utf8(buf)?)
        }
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        if self.len() > LENGTH_LIMIT {
            Err(anyhow!("String too long to write"))
        } else {
            w.write_all(self.as_bytes())?;
            Ok(())
        }
    }
}
