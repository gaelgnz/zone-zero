use crate::{item::{Weapon}, map::{Map, TileKind}};
use macroquad::prelude::*;
use macroquad::rand::*;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io, process};

const TILE_SIZE: f32 = 32.0;
const MAP_WIDTH: u32 = 256;
const MAP_HEIGHT: u32 = 256;
#[derive(PartialEq)]
enum DrawMode {
    Tiles,
    Items,
}

#[macroquad::main("Mapping Tool")]
pub async fn main() {
    srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);
    let mut map: Map;
    

    let mut buf = String::new();
    println!("Load map? y/n");
    io::stdin().read_line(&mut buf).unwrap();
    match buf.trim() {
        "y" => {
            map = serde_json::from_str(&fs::read_to_string("map.json").unwrap()).unwrap();
        }
        &_ => { 
            map = Map::new(MAP_WIDTH, MAP_HEIGHT);
        }
    }

    let mut tile_kind = TileKind::Grass;
    let mut drawing_mode = DrawMode::Tiles;
    let mut can_collide = true;

    let mut cx = 0.0;
    let mut cy = 0.0;
    loop {
        clear_background(WHITE);
        let camera = Camera2D {
            target: vec2(cx, cy),
            zoom: vec2(1.0 / screen_width() * 1.0, 1.0 / screen_height() * 1.0),
            ..Default::default()
        };
        set_camera(&camera);
        if is_key_down(KeyCode::W) {
            cy -= 1.0;
        }
        if is_key_down(KeyCode::S) {
            cy += 1.0;
        }
        if is_key_down(KeyCode::A) {
            cx -= 1.0;
        }
        if is_key_down(KeyCode::D) {
            cx += 1.0;
        }
        for (y, row) in map.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile.kind {
                    TileKind::Grass => draw_rectangle(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        GREEN,
                    ),
                    TileKind::Rock => draw_rectangle(
                        x as f32 * TILE_SIZE,
                        y as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                        BLACK,
                    ),
                    TileKind::Empty => {}
                }
            }
        }
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Mapping Tool").show(egui_ctx, |ui| {
                ui.heading("Current mode");
                ui.radio_value(&mut drawing_mode, DrawMode::Tiles, "Tiles");
                ui.radio_value(&mut drawing_mode, DrawMode::Items, "Items");
                ui.heading("Drawing Mode");
                ui.checkbox(&mut can_collide, "Can collide");
                ui.radio_value(&mut tile_kind, TileKind::Rock, "Rock");
                ui.radio_value(&mut tile_kind, TileKind::Grass, "Grass");
                ui.radio_value(&mut tile_kind, TileKind::Empty, "Empty");

                if ui.button("Save and quit").clicked() {
                    fs::write("map.json", serde_json::to_string(&map).unwrap()).unwrap();
                    process::exit(0);
                }
            });
        });
        egui_macroquad::draw();
        match drawing_mode {
            DrawMode::Tiles => {
                if is_mouse_button_down(MouseButton::Left) || is_mouse_button_down(MouseButton::Right) {
                    let mouse_screen = vec2(mouse_position().0, mouse_position().1);
                    let mouse_world = camera.screen_to_world(mouse_screen);

                    let x = (mouse_world.x / TILE_SIZE).floor() as usize;
                    let y = (mouse_world.y / TILE_SIZE).floor() as usize;

                    if x < MAP_WIDTH as usize && y < MAP_HEIGHT as usize {
                        match tile_kind {
                            TileKind::Rock => {
                                map.tiles[y][x].kind = TileKind::Rock;
                                map.tiles[y][x].collision = can_collide;
                            }
                            TileKind::Grass => {
                                map.tiles[y][x].kind = TileKind::Grass;
                                map.tiles[y][x].collision = can_collide;
                            }
                            TileKind::Empty => {
                                map.tiles[y][x].kind = TileKind::Empty;
                                map.tiles[y][x].collision = false;
                            }
                        }
                    }
                }
            },
            DrawMode::Items => {

                let mouse_screen = vec2(mouse_position().0, mouse_position().1);
                let mouse_world = camera.screen_to_world(mouse_screen);

                if is_mouse_button_pressed(MouseButton::Left) {
                    map.items.push(Weapon::ak47(mouse_world.x, mouse_world.y, false));
                }
            }
        }
        next_frame().await;
    }
}
