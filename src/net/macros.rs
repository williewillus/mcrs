macro_rules! packet_struct {
    // Standard payload
    ($name:ident { $($fieldname:ident: $fieldtype:ty),+ }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fieldname: $fieldtype),*
        }

        impl ProtoSerializable for $name {
            
            fn read<R: Read>(mut r: R) -> Result<Self>
            where Self: Sized {
                let pkt = Self {
                    $($fieldname: proto::read::<$fieldtype, _>(&mut r)?),+
                };
                Ok(pkt)
            }

            fn write<W: Write>(&self, mut w: W) -> Result<()> {
                $(self.$fieldname.write(&mut w)?;)+
                Ok(())
            }
        }
    };

    // No payload
    ($name:ident {}) => {
        #[derive(Debug)]
        pub struct $name;

        impl crate::net::proto::ProtoSerializable for $name {
            
            fn read<R: Read>(_: R) -> Result<Self>
            where Self: Sized {
                Ok(Self)
            }

            fn write<W: Write>(&self, _: W) -> Result<()> {
                Ok(())
            }
        }
    };

    // Custom ProtoSerializable impl
    ($name:ident { $($fieldname:ident: $fieldtype:ty),+; $ps_impl:item }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $fieldname: $fieldtype),*
        }

        $ps_impl
    }
}

macro_rules! packets {
    ($($id:expr => $name:ident { $($payload:tt)* }),+) => {
        use ::anyhow::Result;
        use ::std::convert::TryFrom;
        use ::std::io::{Read, Write};
        use $crate::net::proto::{self, ProtoSerializable};
        use $crate::net::packet::RawPacket;
        
        $( packet_struct!($name { $($payload)* }); )*


        pub enum Packet {
            $($name($name)),+
        }

        impl TryFrom<RawPacket> for Packet {
            type Error = ::anyhow::Error;
            
            fn try_from(pkt: RawPacket) -> Result<Self> {
                let mut payload = pkt.data.as_slice();
                let res = match pkt.packet_id {
                    $($id => { Packet::$name($name::read(&mut payload)?) }),+
                    i @ _ => return Err(::anyhow::anyhow!("Unknown packet discriminator {}", i))
                };
                Ok(res)
            }
        }

        impl Packet {
            pub fn write<W: Write>(&self, mut w: W) -> Result<()> {
                let mut buf = Vec::new();

                match self {
                    $(Packet::$name(payload) => {
                        proto::write_varint(&mut buf, $id)?;
                        payload.write(&mut buf)?;
                    },)+
                }

                let len = buf.len() as i32;
                proto::write_varint(&mut w, len)?;
                Ok(w.write_all(buf.as_slice())?)
            }
        }
    };
}
