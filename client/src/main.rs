
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
                            // Here you would typically initiate a connection to the server
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