pub struct Player {
    pub x: f32,
    pub y: f32,
}

pub struct Map {
    pub content: Vec<Vec<u8>>,
    pub red_spawn: f32,
    pub blue_spawn: f32,
}