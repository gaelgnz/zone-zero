use macroquad::prelude::*;
use macroquad::audio::{load_sound, Sound};

pub struct Resources {
    pub player_texture: Texture2D,

    pub weapon_ak47_texture_picked: Texture2D,
    pub weapon_ak47_texture: Texture2D,
    pub weapon_ak47_shot_sound: Sound,

    pub weapon_magnum_texture_picked: Texture2D,

    pub chat_sound: Sound,
}

impl Resources {
    pub async fn load() -> Self {
        let player_texture = load_texture("res/player.png").await.unwrap();

        let weapon_ak47_texture_picked = load_texture("res/weapon_ak47_picked.png").await.unwrap();
        let weapon_ak47_texture = load_texture("res/weapon_ak47.png").await.unwrap();
        let weapon_ak47_shot_sound = load_sound("res/weapon_ak47_shot.wav").await.unwrap();

        let weapon_magnum_texture_picked = load_texture("res/weapon_magnum.png").await.unwrap();

        let chat_sound = load_sound("res/chat.wav").await.unwrap();

        Self {
            player_texture,
            weapon_ak47_texture_picked,
            weapon_ak47_texture,
            weapon_ak47_shot_sound,
            weapon_magnum_texture_picked,
            chat_sound,
        }
    }
}
