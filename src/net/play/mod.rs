pub mod clientbound {
    use crate::types::text::Text;
    
    packets! {
        0x1B => Disconnect { reason: Text },
        0x21 => KeepAlive { value: i64 }
    }
}

pub mod serverbound {
    packets! {
        0x03 => Chat { message: String },
        0x0F => KeepAliveResponse { value: i64 },
        0x11 => PlayerPosition { x: f64, y: f64, z: f64, on_ground: bool },
        0x12 => PlayerPositionAndRotation { x: f64, y: f64, z: f64, yaw: f32, pitch: f32, on_ground: bool },
        0x13 => PlayerRotation { yaw: f32, pitch: f32, on_ground: bool },
        0x14 => PlayerMovement { on_ground: bool }
    }
}
