use env_logger::Env;
use log::{error, info};
use packetio::{PacketReceiver, PacketSender};

use shared::{
    packets::{JoinPacket, JoinPacketServerReply, PlayerUpdatePacket},
    Game, Player, PlayerActions, Server,
};
use std::{
    net::{TcpListener, TcpStream}, sync::{Arc, Mutex}, thread
};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let listener: TcpListener = match TcpListener::bind("0.0.0.0:9123") {
        Ok(l) => l,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    let server: Arc<Mutex<Server>> = Arc::new(Mutex::new(Server {
        listener: listener,
        game: Game {
            state: shared::LobbyState::Intermission,
            red_score: 0,
            blue_score: 0,
        },
    }));

    info!("Server started on port 9123, Waiting for connections...");

    for stream in server.lock().unwrap().listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("Stream connected: {}", stream.peer_addr().unwrap());
                let server = server.clone();
                thread::spawn(move || {
                    handle_client(stream, server);
                });
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    };
}

fn handle_client(mut stream: TcpStream, server: Arc<Mutex<Server>>) {
    let join_packet: JoinPacket = stream.recv_packet().unwrap();

    println!("{:?}", join_packet);

    let reply: JoinPacketServerReply = JoinPacketServerReply {
        rejected: false,
        player: Some(Player {
            name: join_packet.username,
            team: true,
            x: 0.0,
            y: 0.0,
        })
    };

    stream.send_packet(reply);

    loop {
        let packet: PlayerUpdatePacket = match stream.recv_packet() {
            Ok(p) => p,
            Err(e) => {
                error!("Error receiving packet: {}", e);
                thread::current().unpark();
                break;
            }
        };
        println!("{:?}", packet);
    }
}
