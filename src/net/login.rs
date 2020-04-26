pub mod serverbound {
    packets! {
        0 => LoginStart { name: String }
    }
}

pub mod clientbound {
    use std::str::FromStr;
    
    packets! {
        0 => Disconnect { reason: crate::types::text::Text },
        2 => LoginSuccess { uuid: uuid::Uuid, name: String;
                            impl ProtoSerializable for LoginSuccess {
                                fn read<R: Read>(mut r: R) -> Result<Self> {
                                    let us = proto::read::<String, _>(&mut r)?;
                                    let uuid = uuid::Uuid::from_str(&us)?;
                                    let name = proto::read(&mut r)?;
                                    Ok(Self { uuid, name })
                                }

                                fn write<W: Write>(&self, mut w: W) -> Result<()> {
                                    self.uuid.to_string().write(&mut w)?;
                                    self.name.write(&mut w)?;
                                    Ok(())
                                }
                            }
        }
    }
}
