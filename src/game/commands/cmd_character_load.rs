use prost::Message;
use crate::game::player::Player;
use crate::game::commands::cmd_base::CmdBase;
use crate::proto_gen::game_service_proto::{Character, CharactersLoadReq, game_service_message, GameServiceMessage, GameServiceMessageTypeE, Motion, XyzVector};
use crate::proto_gen::game_service_proto::CharactersLoadResp;
use crate::proto_gen::game_service_proto::game_service_message::Payload::CharactersLoadRequest;

pub struct CmdCharacterLoad;

impl CmdCharacterLoad {
    fn encode(resp: CharactersLoadResp) -> Vec<u8> {
        let game_service_message = GameServiceMessage {
            id: GameServiceMessageTypeE::CharactersLoadRespE as i32,
            payload: Some(game_service_message::Payload::CharactersLoadResponse(resp)),
        };

        let mut buffer = Vec::new();
        game_service_message.encode(&mut buffer).unwrap();
        buffer
    }

    fn handle(req: CharactersLoadReq) -> Vec<u8> {
        println!("Received Character Load Request");
        let player_token = req.token;
        let player = Character {
            id: "character1".to_string(),
            instance_id: "instance1".to_string(),
            map_id: "map1".to_string(),
            motion: Option::from(Motion { map_name: String::from("default"), position: None, rotation: None, velocity: None }), // Adjust this based on the actual definition of Motion
        };
        let characters_load_resp = CharactersLoadResp {
            result: true,
            player: Option::from(player),
            characters: vec![],
        };
        CmdCharacterLoad::encode(characters_load_resp)
    }
}
impl CmdBase for CmdCharacterLoad {

    fn exec(&self, character: &mut Player, msg: GameServiceMessage) -> Vec<u8> {
        match msg.payload.unwrap() {
            CharactersLoadRequest(req) => Self::handle(req),
            _ => vec![],
        }
    }
}

impl CmdCharacterLoad {
    pub fn new() -> Self {
        CmdCharacterLoad
    }
}
