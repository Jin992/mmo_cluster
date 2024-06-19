use prost::Message;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::proto_gen::game_service_proto::GameServiceMessage;
use crate::proto_gen::game_service_proto::GameServiceMessageTypeE;
use crate::game::player::Player;

use crate::game::commands::cmd_base::CmdBase;
use crate::game::commands::cmd_character_load::CmdCharacterLoad;
use crate::game::commands::cmd_character_update::CmdCharacterUpdate;
use std::collections::HashMap;


pub struct GameSession {
    pub socket: TcpStream,
    pub actions: HashMap<GameServiceMessageTypeE, Box<dyn CmdBase + Send>>,
}

impl GameSession {
    pub fn new(socket: TcpStream) -> Self {
        let mut actions: HashMap<GameServiceMessageTypeE,  Box<dyn CmdBase + Send>> = HashMap::new();
        actions.insert(GameServiceMessageTypeE::CharactersLoadReqE, Box::new(CmdCharacterLoad::new()));
        actions.insert(GameServiceMessageTypeE::CharactersUpdateReqE, Box::new(CmdCharacterUpdate::new()));
        GameSession{socket, actions}
    }
    async fn handle_player(&mut self, player:& mut Player, buf: Vec<u8>) -> Vec<u8> {
        let msg = GameServiceMessage::decode(&*buf).expect("Failed to decode message");
        self.actions[&msg.id()].exec(player)
    }

    pub async fn start(&mut self) {
        let mut buf: Vec<u8> = vec![0; 4096];
        let mut player= Player::new();

        loop {
            let mut buf = vec![0; 4096];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = self.socket.read(&mut buf).await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }
                buf = self.handle_player(&mut player, buf).await;
                self.socket.write_all(&buf).await
                    .expect("failed to write data to socket");
            }
        }

    }
}