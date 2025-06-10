use std::time::Instant;

use macroquad::{miniquad::TextureFormat, prelude::ImageFormat, rand::gen_range, texture::Texture2D};
use serde::{Serialize, Deserialize};
use bincode::{Encode, Decode};
#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub enum WeaponKind {
    Ak47,
    Magnum

}

#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub enum AmmoType {
    Small,
    Medium,
    Large,
}
#[derive(Serialize, Deserialize, Clone, Encode, Decode, Debug, PartialEq)]
pub struct Weapon {
    pub weapon_kind: WeaponKind,
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
    pub shotoffset: (f32, f32)
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
                weapon_kind: WeaponKind::Ak47,
                damage: 13,
                bullets_per_shot: 1,
                magazine: 30,
                magazine_size: 30,
                reload_time: 2,
                spread: 10.0,
                is_auto: true,
                firerate: 0.1,
                last_shot_time: 0.0,
                ammo_type: AmmoType::Medium,
                shotoffset: (20.0, 8.0)
            }),
        }
    }
    pub fn magnum(px: f32, py: f32, ppicked: bool) -> Item {
        Item {
            id: gen_range(u64::MIN, u64::MAX),
            x: px,
            y: py,
            picked: ppicked,
            name: String::from(""),
            texture: Some("res/weapon_magnum.png".to_string()),
            texture_equipped: Some("res/weapon_magnum_picked.png".to_string()),
            kind: ItemKind::Weapon(Weapon {
                weapon_kind: WeaponKind::Magnum,
                damage: 40,
                bullets_per_shot: 1,
                magazine: 6,
                magazine_size: 6,
                reload_time: 4,
                spread: 2.0,
                is_auto: true,
                firerate: 0.1,
                last_shot_time: 0.0,
                ammo_type: AmmoType::Medium,
                shotoffset: (20.0, 8.0)
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
