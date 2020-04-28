use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use crate::net::proto::ProtoSerializable;

#[derive(Debug)]
pub struct Varint(pub i32);

impl From<i32> for Varint {
    fn from(v: i32) -> Self {
        Self(v)
    }
}

impl ProtoSerializable for Varint {
    fn read<R: Read>(mut r: R) -> Result<Self> {
        let mut buf = [0u8];
        let mut result = 0i32;

        for i in 0..5 {
            r.read_exact(&mut buf)?;
            let val = buf[0] & 0b0111_1111;
            let has_more = buf[0] & 0b1000_0000 != 0;
            if i == 4 && has_more {
                return Err(anyhow!("VarInt too long"));
            } else {
                result |= (val as i32) << (7 * i);
                if !has_more {
                    break;
                }
            }
        }
        Ok(Varint(result))
    }

    fn write<W: Write>(&self, mut w: W) -> Result<()> {
        let mut val = self.0 as u32; // so we get zero-extended shifts
        for _ in 0..5 {
            let mut b = (val & 0b0111_1111) as u8;
            val >>= 7;
            let has_more = val != 0;
            b |= if has_more { 1 } else { 0 } << 7;

            w.write_all(&[b])?;

            if !has_more {
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Varint;
    use crate::net::proto::{self, ProtoSerializable};

    const CASES: &[(i32, &[u8])] = &[
        (0, &[0]),
        (1, &[1]),
        (2, &[2]),
        (127, &[0x7f]),
        (128, &[0x80, 0x01]),
        (255, &[0xff, 0x01]),
        (2147483647, &[0xff, 0xff, 0xff, 0xff, 0x07]),
        (-2147483648, &[0x80, 0x80, 0x80, 0x80, 0x08]),
        (-1, &[0xff, 0xff, 0xff, 0xff, 0x0f]),
    ];

    #[test]
    fn test_read() {
        for (val, bytes) in CASES {
            let mut bytes = *bytes;
            assert_eq!(proto::read_varint(&mut bytes).unwrap(), *val);
        }
    }

    #[test]
    fn test_read_too_long() {
        let mut bytes: &[u8] = &[0x80, 0x80, 0x80, 0x80, 0x80];
        assert!(proto::read_varint(&mut bytes).is_err());
    }

    #[test]
    fn test_write() {
        let mut buf = Vec::new();
        for (val, bytes) in CASES {
            buf.clear();
            let v = Varint(*val);
            v.write(&mut buf).unwrap();
            assert_eq!(buf, *bytes);
        }
    }
}
