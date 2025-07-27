
use std::net::TcpStream;

use macroquad::{prelude::*, ui::{hash, root_ui}};
use shared;

enum GameState {
    Menu,
    Playing,
}

#[macroquad::main("Zone Zero")]
async fn main() {
    let window_size = vec2(400., 400.);
    let mut ip = String::new();
    let mut game_state = GameState::Menu;

    let mut connection: Option<TcpStream> = None;

    loop {
        match game_state {
            GameState::Menu => {
                let position = vec2(
                    (screen_width() - window_size.x) / 2.0,
                    (screen_height() - window_size.y) / 2.0,
                );
                clear_background(BLACK);

                root_ui().window(hash!(),  position, window_size, |ui| {
                    ui.input_text(hash!(), "Ip", &mut ip);
                    if ui.button(None, "Connect") {
                        if !ip.is_empty() {
                            info!("Connecting to {}", ip);
                            
                            match TcpStream::connect(ip.clone()) {
                                Ok(stream) => {
                                    connection = Some(stream);
                                    info!("Connected to {}", ip);
                                },
                                Err(e) => {
                                    info!("Failed to connect: {}", e);
                                }
                            }

                            game_state = GameState::Playing;
                        } else {
                            info!("Please enter a valid IP address.");
                        }
                    }
                });
            }
            GameState::Playing => {

            }
        }

        next_frame().await;
    }
}