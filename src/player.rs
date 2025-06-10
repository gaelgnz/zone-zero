use std::u64;

use bincode::{Decode, Encode};
use macroquad::rand::gen_range;

use crate::item::{Item, WeaponKind};
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Decode, Encode)]
pub enum ActionType {
    Shot((WeaponKind, f32, f32, f32, f32)),
    PickUp(u64)
}


#[derive(Debug)]
pub struct Player {
    pub health: u32,
    pub name: String,
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub dir: bool,
    pub message: String,
    pub current_item: usize,
    pub items: Vec<Item>,
    pub actions: Vec<ActionType>
}

impl Player {
    pub fn new(name: String, x: f32, y: f32) -> Self {
        Player {
            health: 100,
            id: gen_range(u64::MIN, u64::MAX),
            name,
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            dir: false,
            message: String::new(),
            current_item: 0,
            items: Vec::new(),
            actions: Vec::new()
        }
    }
}

impl Player {
    pub fn from_player_packet(packet: &crate::packet::PlayerPacket) -> Self {
        Self {
            health: 100,
            id: gen_range(u64::MIN, u64::MAX),
            name: packet.name.to_string(),
            x: packet.x,
            y: packet.y,
            vx: 0.0,
            vy: 0.0,
            dir: packet.dir,
            message: packet.message.clone(),
            current_item: 0,
            items: Vec::new(),
            actions: Vec::new(),
        }
    }
}
