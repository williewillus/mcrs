use anyhow::{anyhow, Result};
use crate::net::proto::{ProtoSerializable, self};
use std::io::{Read, Write};

const LENGTH_LIMIT: usize = 32767;

// TODO: Split protoserializable into read/write halves and implement the write half for &str?

impl ProtoSerializable for String {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let len = proto::read_varint(&mut r)?;
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
            proto::write_varint(&mut w, self.len() as i32)?;
            w.write_all(self.as_bytes())?;
            Ok(())
        }
    }
}


#[cfg(test)]
mod test {
    use crate::net::proto::{self, ProtoSerializable};
    
    #[test]
    fn test_string_roundtrip() {
        let test_string = "hello world! have some utf8 too: 祈華夢".to_string();
        let mut buf = Vec::new();
        test_string.write(&mut buf).unwrap();
        let roundtripped = proto::read::<String, _>(buf.as_slice()).unwrap();
        assert_eq!(test_string, roundtripped);
    }
}
