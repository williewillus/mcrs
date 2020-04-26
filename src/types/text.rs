use anyhow::Result;
use crate::net::proto::{self, ProtoSerializable};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Text {
    pub text: String
}

// TODO: temporary for status check. Support all text types later.
impl ProtoSerializable for Text {
    fn read<R: Read>(mut r: R) -> Result<Self>
    where Self: Sized {
        let s = proto::read::<String, _>(&mut r)?;
        Ok(serde_json::from_str(&s)?)
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let s = serde_json::to_string(self)?;
        s.write(&mut w)?;
        Ok(())
    }
}
