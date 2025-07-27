pub struct Player {
    pub name: String,
    pub team: bool, // blue = trve
    pub x: f32,
    pub y: f32,
}

pub struct Map {
    pub content: Vec<Vec<u8>>,
    pub red_spawn: f32,
    pub blue_spawn: f32,
}

pub trait Entity {
    fn spawn(&self, on: (f32, f32)) {}
    fn move_to(&mut self, to: (f32, f32)) {}
}

mod entities {
    pub struct Bullet {
        pub x: f32,
        pub y: f32,
        pub damage: f32,
        pub speed: f32,
    }

    
}

impl Entity for entities::Bullet {
    fn move_to(&mut self, to: (f32, f32)) {
        self.x = to.0;
        self.y = to.1;
    }
}

pub struct Game {
    pub red_score: u8,
    pub blue_score: u8,
    pub entities: Vec<Box<dyn Entity>>
}