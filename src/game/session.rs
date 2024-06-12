use mini_redis::{Connection,};
use tokio::net::TcpStream;
use protobuf::Message;
use crate::proto_gen::game_service_proto::GameServiceMessage;
use crate::proto_gen::game_service_proto::GameServiceMessageTypeE;

use crate::game::char::character::Character;
use crate::game::commands::cmd_base::{CmdBase, CommandId};
use crate::game::commands::cmd_character_load::CmdCharacterLoad;
use crate::game::commands::cmd_character_update::CmdCharacterUpdate;

async fn handle_character(character_ref: &mut Character, cmd: Box<dyn CmdBase + Send>) {
    cmd.exec(character_ref);
}

pub struct GameSession {
    pub connection: Connection
}

impl GameSession {
    pub fn new(socket: TcpStream) -> Self {
        GameSession{connection: Connection::new(socket)}
        socket
    }

    pub async fn start(&mut self) {
        let mut player = Character::new();

        while let Some(frame) = self.connection.read_frame().await.unwrap() {
            println!("GOT: {:?}", frame);
            // Message::parse_from_bytes(frame.)
            GameServiceMessage::try_from(frame.to_string());

            let cmd_id = CommandId::CmdCharacterLoad;
            match cmd_id {
                CommandId::CmdCharacterLoad=> {
                    handle_character(&mut player, Box::new(CmdCharacterLoad::new())).await;
                },
                CommandId::CmdCharacterUpdate=> {
                    handle_character(&mut player, Box::new(CmdCharacterUpdate::new())).await;
                },
                _=>println!("Unknown commnd"),
            }


            // Respond with an error
            //let response = Frame::Error("unimplemented".to_string());
            //connection.write_frame(&response).await.unwrap();
            self.connection.write_frame(&frame).await.unwrap();
        }

    }
}