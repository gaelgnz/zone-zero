#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub message: String,
    pub rotation: f32,
}

impl Player {
    pub fn new(id: String, x: f32, y: f32) -> Self {
        Player {
            health: 100,
            id,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            message: String::new(),
            rotation: 0.0,
        }
    }
}

impl Player {
    pub fn from_player_packet(packet: &crate::packet::PlayerPacket) -> Self {
        Self {
            health: 100,
            id: packet.id.to_string(),
            x: packet.x,
            y: packet.y,
            vx: 0.0,
            vy: 0.0,

            message: packet.message.clone(),
            rotation: packet.rotation,
        }
    }
}
