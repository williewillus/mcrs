pub mod clientbound {
    use crate::net::proto::varint::Varint;
    use crate::types::text::Text;
    
    packets! {
        0x1B => Disconnect { reason: Text },
        0x21 => KeepAlive { value: i64 },
        0x26 => JoinGame { entity_id: i32, gamemode: u8, dimension: i32, hashed_seed: i64, max_players: u8, level_type: String,
                           view_dist: Varint, reduced_debug: bool, respawn_screen: bool },
        0x41 => UpdateViewPosition { x: Varint, z: Varint }
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
