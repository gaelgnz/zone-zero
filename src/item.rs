use std::time::Instant;

use macroquad::{miniquad::TextureFormat, prelude::ImageFormat, rand::gen_range, texture::Texture2D};
use serde::{Serialize, Deserialize};
use bincode::{Encode, Decode};
#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub enum AmmoType {
    Small,
    Medium,
    Large,
}
#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub struct Weapon {
    pub damage: u32,
    pub bullets_per_shot: u32,
    pub magazine: u32,
    pub magazine_size: u32,
    pub reload_time: u32,
    pub spread: f32,
    pub is_auto: bool,
    pub firerate: f32,
    pub last_shot_time: f32,
    pub ammo_type: AmmoType,
}
impl Weapon {
    pub fn ak47(px: f32, py: f32, ppicked: bool) -> Item {
        Item {
            id: gen_range(u64::MIN, u64::MAX),
            x: px,
            y: py,
            picked: ppicked,
            name: String::from(""),
            texture: Some("res/weapon_ak47.png".to_string()),
            texture_equipped: Some("res/weapon_ak47_picked.png".to_string()),
            kind: ItemKind::Weapon(Weapon {
                damage: 13,
                bullets_per_shot: 1,
                magazine: 30,
                magazine_size: 30,
                reload_time: 2,
                spread: 2.0,
                is_auto: true,
                firerate: 0.3,
                last_shot_time: 0.0,
                ammo_type: AmmoType::Medium,
            }),
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub enum ItemKind {
    Weapon(Weapon),
    Armor,
    Consumable,
}
#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub struct Item {
    pub id: u64,
    pub x: f32,
    pub y: f32,
    pub picked: bool,
    pub name: String,
    pub texture: Option<String>,
    pub texture_equipped: Option<String>,
    pub kind: ItemKind,
}
