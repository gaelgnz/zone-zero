use crate::common::TILE_SIZE;
use std::fs;
use crate::debugutils::log;
use crate::map::{Map, TileKind};
use crate::packet::{self, PlayerPacket, send_packet};
use crate::player::{ActionType, Player};
use crate::item::{Item, ItemKind};
use macroquad::rand::srand;
use ::rand;
use macroquad::audio::{load_sound, play_sound};
use macroquad::prelude::*;
use std::default::Default;
use std::io::{Read, Write};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};
use std::{process, thread};

use macroquad::ui::{hash, root_ui, widgets};

static PLAYER_WIDTH: f32 = 50.0;
static PLAYER_HEIGHT: f32 = 50.0;

enum GameInputState {
    Chat,
    Movement,
    Menu,
}

fn conf() -> Conf {
    Conf {
       window_title: "Zone zero".to_string(),
       fullscreen: false,
       high_dpi: false,
       ..Default::default() 
    }
}

#[macroquad::main(conf)]
pub async fn main() {
    srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64);

    let mut game_input_state = GameInputState::Movement;

    println!("Loading assets...");
    let player_texture =
        Texture2D::from_file_with_format(include_bytes!("../res/player.png"), None);
    let chat_sound = load_sound("res/chat.wav").await.unwrap();
    let _ = load_sound("res/stal.wav").await.unwrap();

    println!("Connecting to server...");
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut size_buf = [0u8; 4];
    stream.read_exact(&mut size_buf).unwrap();
    let map_size = u32::from_be_bytes(size_buf) as usize;
    println!("Map size: {}", map_size);
    stream.write_all(b"Ok, received map size").unwrap();
    thread::sleep(Duration::from_millis(100));
    let mut map_buf = vec![0u8; map_size];
    stream.read_exact(&mut map_buf).unwrap();

    let mut map: Map = match bincode::decode_from_slice(&map_buf, bincode::config::standard()) {
        Ok((map, _)) => map,
        Err(e) => {
            panic!(
                "Failed to decode map: {}\nProbably server and client version mismatch",
                e
            )
        }
    };
    println!("Map fetched");

    let random_name = rand::random::<char>();
    let mut player = Player::new(random_name.to_string(), 0.0, 0.0);

    let mut will_send: u8 = 5;
    let player_packets: Arc<Mutex<Vec<PlayerPacket>>> = Arc::new(Mutex::new(Vec::new()));
    let mut time_played = 0.0;

    let mut pre_message = String::new();
    let mut delete_message_timer = 0.0;

    let mut last_shot_time: Instant = Instant::now();

    let mut frame_counter: i128 = 0;

    loop {
        let player_rect = Rect::new(player.x, player.y, 64.0, 64.0);
        player.actions.clear();

        if delete_message_timer <= 0.0 {
            player.message.clear();
            pre_message.clear();
        } else {
            delete_message_timer -= get_frame_time();
        }

        let camera = Camera2D {
            target: vec2(player.x, player.y),
            zoom: vec2(1.0 / screen_width() * 3.0, 1.0 / screen_height() * 3.0),
            ..Default::default()
        };
        set_camera(&camera);


        clear_background(WHITE);

        // Receive packets
        match stream.try_clone() {
            Ok(mut clone) => {
                clone.set_nonblocking(true).ok();
                match packet::receive_packet(&mut clone) {
                    Ok(packet) => {
                        let mut packets = player_packets.lock().unwrap();
                        if let Some(existing) = packets.iter_mut().find(|p| p.id == packet.id) {
                            log(frame_counter, 10, format!("Recieving packages correctly from {}", packet.id).as_str());
                            *existing = packet;
                        } else {
                            println!("Received packet from {}, {:?}", packet.id, packet);
                            packets.push(packet);
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                    Err(e) => println!("Receive error: {}", e),
                }
                clone.set_nonblocking(false).ok();
            }
            Err(e) => println!("Stream clone error: {}", e),
        }
        

        let packets = player_packets.lock().unwrap().clone();

        for packet in packets {
           for action in packet.actions {
                match action {
                    ActionType::PickUp(id) => {
                        println!("retaining item..");
                        map.items.retain(|item| item.id != id);

                    }
                }
           } 
        }
        // === Drawing ===
        

        draw_text(
            pre_message.as_str(),
            player.x - 40.0,
            player.y - 80.0,
            20.0,
GRAY,
        );

        render_player(&player, &player_texture).await;
        render_players(player_packets.lock().unwrap().clone(), &player_texture).await;

        // === Map and Objects Rendering ===
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

 


        for item in &mut map.items {
            let item_rect = Rect::new(item.x, item.y, 32.0, 32.0);
           

            if is_key_pressed(KeyCode::E) && player_rect.overlaps(&item_rect) {
                player.items.push(Item { id: item.id, x: item.x, y: item.y, picked: false, name: item.name.clone(), texture: item.texture.clone(), texture_equipped: item.texture_equipped.clone(), kind: item.kind.clone() });
                    player.actions.push(ActionType::PickUp(item.id));            
            }

            draw_texture_ex(
                &Texture2D::from_file_with_format(include_bytes!("../res/weapon_ak47.png"), None),
                item.x,
                item.y,
                WHITE,
                DrawTextureParams {
                    ..Default::default()
                },
            );
        }

        for action in &player.actions {
            match action {
                ActionType::PickUp(id) => {
                    map.items.retain(|x| x.id != *id);
                }
            }
        }
            



        match game_input_state {
            GameInputState::Movement => {
                if !player.items.is_empty() {

                    if let ItemKind::Weapon(ref mut weapon) = player.items[player.current_item].kind {
                        // Calculate minimum time between shots
                        let shot_interval = Duration::from_secs_f32(1.0 / weapon.firerate);

                        // Check if enough time passed since last shot
                        let can_shoot = Instant::now().duration_since(last_shot_time) >= shot_interval;

                        // Check if mouse pressed for semi-auto or just is_auto for auto mode
                        let firing_condition = if weapon.is_auto {
                            is_mouse_button_down(MouseButton::Left) && can_shoot
                        } else {
                            // Semi-auto mode: trigger only on mouse button press
                            is_mouse_button_pressed(MouseButton::Left) && can_shoot
                        };

                        if firing_condition {
                            if weapon.magazine >= weapon.bullets_per_shot {
                                let mouse_world = camera.screen_to_world(vec2(mouse_position().0, mouse_position().1)); 
                                
                                last_shot_time = Instant::now();
                                weapon.magazine -= weapon.bullets_per_shot;
                                draw_line(player.x, player.y, mouse_world.x, mouse_world.y, 5.0, YELLOW);
                            } else {
                            
                            }
                        }
                    }
                }

                if is_key_pressed(KeyCode::Key1) {
                    player.current_item = 1;
                }
                if is_key_pressed(KeyCode::Key2) {
                    player.current_item = 2;
                }
                if is_key_pressed(KeyCode::T) {
                    game_input_state = GameInputState::Chat;
                }

                if is_key_down(KeyCode::A) {
                    player.vx = -5.0;
                    player.dir = false;
                } else if is_key_down(KeyCode::D) {
                    player.vx = 5.0;
                    player.dir = true;
                } else {
                    player.vx = 0.0;
                }
                if is_key_down(KeyCode::W) {
                    player.vy = -5.0;
                } else if is_key_down(KeyCode::S) {
                    player.vy = 5.0;
                } else {
                    player.vy = 0.0;
                }

                if is_key_pressed(KeyCode::Escape) {
                    game_input_state = GameInputState::Menu;
                }
            }

            GameInputState::Chat => {
                if is_key_pressed(KeyCode::Backspace) {
                    pre_message.pop();
                }
                if is_key_pressed(KeyCode::Space) {
                    pre_message.push(' ');
                }

                let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);

                {
                    if is_key_pressed(KeyCode::A) {
                        pre_message.push(if shift { 'A' } else { 'a' });
                    }
                    if is_key_pressed(KeyCode::B) {
                        pre_message.push(if shift { 'B' } else { 'b' });
                    }
                    if is_key_pressed(KeyCode::C) {
                        pre_message.push(if shift { 'C' } else { 'c' });
                    }
                    if is_key_pressed(KeyCode::D) {
                        pre_message.push(if shift { 'D' } else { 'd' });
                    }
                    if is_key_pressed(KeyCode::E) {
                        pre_message.push(if shift { 'E' } else { 'e' });
                    }
                    if is_key_pressed(KeyCode::F) {
                        pre_message.push(if shift { 'F' } else { 'f' });
                    }
                    if is_key_pressed(KeyCode::G) {
                        pre_message.push(if shift { 'G' } else { 'g' });
                    }
                    if is_key_pressed(KeyCode::H) {
                        pre_message.push(if shift { 'H' } else { 'h' });
                    }
                    if is_key_pressed(KeyCode::I) {
                        pre_message.push(if shift { 'I' } else { 'i' });
                    }
                    if is_key_pressed(KeyCode::J) {
                        pre_message.push(if shift { 'J' } else { 'j' });
                    }
                    if is_key_pressed(KeyCode::K) {
                        pre_message.push(if shift { 'K' } else { 'k' });
                    }
                    if is_key_pressed(KeyCode::L) {
                        pre_message.push(if shift { 'L' } else { 'l' });
                    }
                    if is_key_pressed(KeyCode::M) {
                        pre_message.push(if shift { 'M' } else { 'm' });
                    }
                    if is_key_pressed(KeyCode::N) {
                        pre_message.push(if shift { 'N' } else { 'n' });
                    }
                    if is_key_pressed(KeyCode::O) {
                        pre_message.push(if shift { 'O' } else { 'o' });
                    }
                    if is_key_pressed(KeyCode::P) {
                        pre_message.push(if shift { 'P' } else { 'p' });
                    }
                    if is_key_pressed(KeyCode::Q) {
                        pre_message.push(if shift { 'Q' } else { 'q' });
                    }
                    if is_key_pressed(KeyCode::R) {
                        pre_message.push(if shift { 'R' } else { 'r' });
                    }
                    if is_key_pressed(KeyCode::S) {
                        pre_message.push(if shift { 'S' } else { 's' });
                    }
                    if is_key_pressed(KeyCode::T) {
                        pre_message.push(if shift { 'T' } else { 't' });
                    }
                    if is_key_pressed(KeyCode::U) {
                        pre_message.push(if shift { 'U' } else { 'u' });
                    }
                    if is_key_pressed(KeyCode::V) {
                        pre_message.push(if shift { 'V' } else { 'v' });
                    }
                    if is_key_pressed(KeyCode::W) {
                        pre_message.push(if shift { 'W' } else { 'w' });
                    }
                    if is_key_pressed(KeyCode::X) {
                        pre_message.push(if shift { 'X' } else { 'x' });
                    }
                    if is_key_pressed(KeyCode::Y) {
                        pre_message.push(if shift { 'Y' } else { 'y' });
                    }
                    if is_key_pressed(KeyCode::Z) {
                        pre_message.push(if shift { 'Z' } else { 'z' });
                    }
                }

                if is_key_pressed(KeyCode::Enter) {
                    player.message = pre_message.clone();
                    pre_message.clear();
                    play_sound(&chat_sound, Default::default());
                    game_input_state = GameInputState::Movement;
                }

                delete_message_timer = 5.0;
            }
            GameInputState::Menu => {
                if is_key_pressed(KeyCode::Escape) {
                    game_input_state = GameInputState::Movement;
                }
                egui_macroquad::ui(|egui_ctx| {
                    egui::Window::new("Momentum").show(egui_ctx, |ui| {
                        ui.label(format!("Time played: {:.2}s", time_played));
                        ui.label(format!("FPS: {}", get_fps()));
                        if ui.button("Return to Game").clicked() {
                            game_input_state = GameInputState::Movement;
                        }
                        if ui.button("Quit game").clicked() {
                            process::exit(0);
                        };
                    });
                });
                egui_macroquad::draw();
            }
        }

        // == Send Packets ==
        if will_send <= 1 {
            let packet = PlayerPacket::from_player(&player);
            send_packet(&mut stream, &packet).unwrap();
            will_send = 1;
        } else {
            will_send -= 1;
        }

        widgets::Window::new(hash!(), vec2(0.0, 100.0), vec2(32., 200.))
        .movable(false)
        .titlebar(false)
        .ui(&mut root_ui(), |ui| {
            for (i, item) in player.items.iter().enumerate() {
                println!("{:?}", item.texture.clone());
                ui.texture(Texture2D::from_file_with_format(fs::read(item.texture.clone().expect("Expected texture").as_str()).unwrap().as_slice(), None), 32.0, 32.0);
            }
        });

        handle_collisions(&mut player, &map);
        time_played += get_frame_time();
        frame_counter += 1;
        next_frame().await;
    }
}

async fn render_players(player_packets: Vec<PlayerPacket>, texture: &Texture2D) {
    for player_packet in player_packets {
        let player = Player::from_player_packet(&player_packet);
        render_player(&player, texture).await;
    }
}

async fn render_player(player: &Player, texture: &Texture2D) {
    draw_text(
        &player.name.to_string(),
        player.x - 40.0,
        player.y - 50.0,
        20.0,
        BLACK,
    );


    if !player.items.is_empty() {  
        draw_texture_ex(&load_texture(player.items.clone()[player.current_item].texture_equipped.clone().unwrap().as_str()).await.unwrap(), player.x, player.y, WHITE, DrawTextureParams { 
            flip_x: player.dir,
            ..Default::default() 
        });
    }



    if player.message.chars().next().is_some() {
        draw_text(
            &player.message,
            player.x - 40.0,
            player.y - 80.0,
            20.0,
            BLACK,
        );
    }
    draw_texture_ex(
        texture,
        player.x - PLAYER_WIDTH / 2.0,
        player.y - PLAYER_HEIGHT / 2.0,
        WHITE,
        DrawTextureParams {
            flip_x: player.dir,
            dest_size: Some(vec2(PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        },
    );

}

fn handle_collisions(player: &mut Player, map: &Map) {
    const TILE_SIZE: f32 = 32.0;

    let half_width = PLAYER_WIDTH / 2.0;
    let half_height = PLAYER_HEIGHT / 2.0;

    // --- Horizontal movement ---
    player.x += player.vx;

    let left_tile = ((player.x - half_width) / TILE_SIZE).floor() as isize;
    let right_tile = ((player.x + half_width) / TILE_SIZE).ceil() as isize;
    let top_tile = ((player.y - half_height) / TILE_SIZE).floor() as isize;
    let bottom_tile = ((player.y + half_height) / TILE_SIZE).ceil() as isize;

    for ty in top_tile..bottom_tile {
        for tx in left_tile..right_tile {
            if let Some(tile) = map.get_tile(tx as usize, ty as usize) {
                if tile.collision {
                    let tile_rect = Rect::new(
                        tx as f32 * TILE_SIZE,
                        ty as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    );
                    let player_rect = Rect::new(
                        player.x - half_width,
                        player.y - half_height,
                        PLAYER_WIDTH,
                        PLAYER_HEIGHT,
                    );

                    if player_rect.overlaps(&tile_rect) {
                        if player.vx > 0.0 {
                            // Moving right: push player back to left of tile
                            player.x = tile_rect.x - half_width;
                        } else if player.vx < 0.0 {
                            // Moving left: push player to right of tile
                            player.x = tile_rect.x + TILE_SIZE + half_width;
                        }
                        player.vx = 0.0;
                    }
                }
            }
        }
    }

    // --- Vertical movement ---
    player.y += player.vy;

    let left_tile = ((player.x - half_width) / TILE_SIZE).floor() as isize;
    let right_tile = ((player.x + half_width) / TILE_SIZE).ceil() as isize;
    let top_tile = ((player.y - half_height) / TILE_SIZE).floor() as isize;
    let bottom_tile = ((player.y + half_height) / TILE_SIZE).ceil() as isize;

    for ty in top_tile..bottom_tile {
        for tx in left_tile..right_tile {
            if let Some(tile) = map.get_tile(tx as usize, ty as usize) {
                if tile.collision {
                    let tile_rect = Rect::new(
                        tx as f32 * TILE_SIZE,
                        ty as f32 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    );
                    let player_rect = Rect::new(
                        player.x - half_width,
                        player.y - half_height,
                        PLAYER_WIDTH,
                        PLAYER_HEIGHT,
                    );

                    if player_rect.overlaps(&tile_rect) {
                        if player.vy > 0.0 {
                            // Moving down: push player back up
                            player.y = tile_rect.y - half_height;
                        } else if player.vy < 0.0 {
                            // Moving up: push player down
                            player.y = tile_rect.y + TILE_SIZE + half_height;
                        }
                        player.vy = 0.0;
                    }
                }
            }
        }
    }
}

