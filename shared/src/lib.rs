use bincode::{config::standard, Decode, Encode};
use packetio;
use std::{
    default,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

#[derive(Encode, Decode, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub team: bool, // blue = trve
    pub x: f32,
    pub y: f32,
}
#[derive(Encode)]
pub struct Map {
    pub content: Vec<Vec<u8>>,
    pub red_spawn: f32,
    pub blue_spawn: f32,
}


#[derive(Encode, Decode, Debug)]
pub enum PlayerActions {
    Move { x: f32, y: f32 },
}

pub mod packets {
    use crate::Player;
    use crate::PlayerActions;
    use bincode::{Decode, Encode};

    #[derive(Encode, Decode, Debug)]
    pub struct JoinPacket {
        pub username: String,
    }
    #[derive(Encode, Decode, Debug)]
    pub struct JoinPacketServerReply {
        pub rejected: bool,
        pub player: Option<Player>,
    }
    #[derive(Encode, Decode, Debug)]
    pub struct PlayerUpdatePacket {
        pub player_name: String,
        pub player_actions: Vec<PlayerActions>,
    }
    #[derive(Encode, Decode, Debug)]
    pub struct ServerUpdatePacket {

    }
}

pub mod entities {
    pub struct Bullet {
        pub x: f32,
        pub y: f32,
        pub damage: f32,
        pub speed: f32,
    }
}
pub struct Server {
    pub listener: TcpListener,
    pub game: Game,
}

pub enum LobbyState {
    Intermission,
    Ingame,
}

pub struct Game {
    pub state: LobbyState,
    pub red_score: u8,
    pub blue_score: u8
}
