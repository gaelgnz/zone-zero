use macroquad::{
    prelude::*, rand::gen_range, ui::{hash, root_ui}
};
use packetio::{PacketReceiver, PacketSender};
use shared::{packets::{JoinPacket, JoinPacketServerReply, PlayerUpdatePacket}, Player, PlayerActions};
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use crate::resources::Resources;

mod resources;

enum GameState {
    Menu,
    Playing,
}

#[macroquad::main("Zone Zero")]
async fn main() {
    let resources = Resources::load();
    let window_size = vec2(400., 400.);
    let mut ip = "127.0.0.1:9123".to_string();
    let mut username: String = "Player".to_string();
    let mut last_error: String = String::new();
    let mut game_state = GameState::Menu;

    let mut connection: Option<TcpStream> = None;
    let mut player: Option<Player> = None;
    

    loop {
        match game_state {
            GameState::Menu => {
                let position = vec2(
                    (screen_width() - window_size.x) / 2.0,
                    (screen_height() - window_size.y) / 2.0,
                );
                clear_background(BLACK);

                let connect_result = {
                    let mut connect_triggered = false;

                    root_ui().window(hash!(), position, window_size, |ui| {
                        ui.input_text(hash!(), "Ip", &mut ip);
                        ui.input_text(hash!(), "Username", &mut username);

                        if ui.button(None, "Connect") {
                            connect_triggered = true;
                        }

                        ui.label(None, &last_error);
                    });

                    connect_triggered
                };

                if connect_result {
                    if ip.is_empty() || ip.parse::<SocketAddr>().is_err() {
                        last_error = "Please enter a valid IP address.".to_string();
                    } else if username.len() > 8 {
                        last_error = "Username cant be more than 8 characters long".to_string();
                    } else if username.is_empty() {
                        last_error = "Username cant be nothing".to_string();
                    } else {
                        last_error = format!("Connecting to {}", ip);

                        match TcpStream::connect_timeout(
                            &ip.parse::<SocketAddr>().unwrap(),
                            Duration::from_secs(5),
                        ) {
                            Ok(mut stream) => {
                                connection = Some(stream.try_clone().unwrap());
                                let join_packet = JoinPacket {
                                    username: username.clone(),
                                };

                                stream.send_packet(join_packet);

                                let reply: JoinPacketServerReply = stream.recv_packet().unwrap();
                                player = Some(reply.player.unwrap());

                                game_state = GameState::Playing;
                            }
                            Err(e) => {
                                last_error = format!("Failed to connect: {}", e);
                            }
                        }
                    }
                }
            }
            GameState::Playing => {
                let mut player_actions: Vec<PlayerActions> = Vec::new();

                let mut p = player.take().unwrap();
                let mut c = connection.as_mut().expect("Connection is None");

                let delta = get_frame_time();

                clear_background(LIGHTGRAY);

                set_camera(&Camera2D {
                    target: vec2(p.x, p.y),
                    zoom: vec2(1.0 / screen_width() * 2.0, 1.0 / screen_height() * 2.0),
                    ..Default::default()
                });

                draw_texture(&resources.player_texture, p.x, p.y, WHITE);
                // Removed the extra draw_texture at (0.0, 0.0)

                set_default_camera();

                if is_key_down(KeyCode::W) {
                    p.y -= 200.0 * delta;
                    player_actions.push(PlayerActions::Move { x: p.x, y: p.y });
                }
                if is_key_down(KeyCode::S) {
                    p.y += 200.0 * delta;
                    player_actions.push(PlayerActions::Move { x: p.x, y: p.y });
                }
                if is_key_down(KeyCode::A) {
                    p.x -= 200.0 * delta;
                    player_actions.push(PlayerActions::Move { x: p.x, y: p.y });
                }
                if is_key_down(KeyCode::D) {
                    p.x += 200.0 * delta;
                    player_actions.push(PlayerActions::Move { x: p.x, y: p.y });
                }


                c.send_packet(PlayerUpdatePacket {
                    player_name: username.clone(),
                    player_actions,
                });


                player = Some(p);
            }
        }
        next_frame().await;
    }
}
