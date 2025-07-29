use macroquad::prelude::*;

pub struct Resources {
    pub player_texture: Texture2D,
}

impl Resources {
    pub fn load() -> Resources {
        Resources {
            player_texture: Texture2D::from_file_with_format(
                include_bytes!("../../res/player.png"),
                Some(ImageFormat::Png),
            ),
        }
    }
}
