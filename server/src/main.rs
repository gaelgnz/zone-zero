use std::{net::{TcpListener, TcpStream}, thread};
use env_logger::Env;
use log::{error, info};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let listener: TcpListener = match TcpListener::bind("0.0.0.0:9123") {
        Ok(l) => l,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };
    info!("Server started on port 9123, Waiting for connections...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("Stream connected: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                error!("{}", e);

            }
        }
    }
}

fn handle_client(_stream: TcpStream) {}