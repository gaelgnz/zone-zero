pub enum TileKind {
    Grass,
}

pub struct Tile {
    pub collision: bool,
    pub kind: TileKind,
}

pub struct Map {
    pub height: u32,
    pub width: u32,
    pub tiles: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(height: u32, width: u32) -> Self {
        Map {
            height,
            width,
            tiles: Vec::new(),
        }
    }
}
