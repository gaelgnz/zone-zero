use crate::debugutils::log;
use crate::map::Map;
use crate::packet::{self, MapPacket};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::{process, fs, thread, env};
use sha2::{self, Digest, Sha256, Sha256VarCore};
type ClientList = Arc<Mutex<Vec<TcpStream>>>;

pub fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            process::exit(1);
        }
    };
    let clients: ClientList = Arc::new(Mutex::new(Vec::new()));
    let map: Map = serde_json::from_str(
        String::from_utf8(fs::read("map.json").unwrap())
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let serialized_map = MapPacket { data: map }.serialize();

    let exe = env::current_exe().unwrap();
    let mut sha256 = Sha256::new();
    io::copy(&mut fs::File::open(exe).unwrap(), &mut sha256).unwrap();
    let hash = sha256.finalize();
    println!("Server started! ~ Hash: {:x}", hash);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let stream_clone = stream.try_clone().unwrap();
                let clients = Arc::clone(&clients);
                let map_data = serialized_map.clone();

                clients.lock().unwrap().push(stream_clone);

                thread::spawn(move || {
                    handle_client(stream, clients, map_data);
                });
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}

pub fn handle_client(mut stream: TcpStream, clients: ClientList, map: Vec<u8>) {
    let map_packet_size = map.len();
    println!("Map size: {}", map_packet_size);
    let size_bytes = (map_packet_size as u32).to_be_bytes();

    // Send map size prefix
    stream.write_all(&size_bytes).unwrap();

    // Wait for client ack (optional)
    let mut tmp_buf = vec![0u8; 64];
    let read_res = stream.read(&mut tmp_buf);
    if let Ok(n) = read_res {
        if n > 0 {
            println!("Client says: {}", String::from_utf8_lossy(&tmp_buf[..n]));
        }
    }

    // Send the serialized map bytes
    stream.write_all(&map).unwrap();
    let mut frame_counter = 0;
    loop {
        frame_counter += 1;

        let packet = match packet::receive_packet(&mut stream) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Error receiving packet: {}", e);
                eprintln!("Probably server/client mismatch! please restart!");
                break;
            }
        };
        if let Ok(clients_guard) = clients.lock() {
            log(
            frame_counter,
            3600,
            format!("Recieving packages correctly from {}", packet.id).as_str(),
            );
            log(
            frame_counter,
            10000,
            format!("Conected clients: {}", clients_guard.len()).as_str(),
            );
        } else {
            eprintln!("Failed to lock clients mutex for logging.");
        }


        let sender_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                eprintln!("Could not get sender address: {}", e);
                break;
            }
        };

        let mut clients_lock = clients.lock().unwrap();

        clients_lock.retain(|client| {
            match client.peer_addr() {
                Ok(addr) => {
                    if addr != sender_addr {
                        // Broadcast to other clients
                        match client.try_clone() {
                            Ok(mut cloned_stream) => {
                                if let Err(e) = packet::send_packet(&mut cloned_stream, &packet) {
                                    eprintln!("Error broadcasting to {}: {}", addr, e);
                                    return false; // Remove client on error
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to clone stream: {}", e);
                                return false;
                            }
                        }
                    }
                    true
                }
                Err(e) => {
                    eprintln!("Failed to get client address: {}", e);
                    false
                }
            }
        });
    }

    // Remove disconnected client from list
    let mut clients_lock = clients.lock().unwrap();
    clients_lock.retain(|client| match client.peer_addr() {
        Ok(addr) => addr != stream.peer_addr().unwrap(),
        Err(_) => false,
    });

    println!(
        "Client disconnected. Remaining clients: {}",
        clients_lock.len()
    );
}
