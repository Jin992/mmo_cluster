use prost::Message;
use crate::game::player::Player;
use crate::game::commands::cmd_base::CmdBase;
use crate::proto_gen::game_service_proto::{Character, CharactersLoadResp, CharactersUpdateReq, CharactersUpdateResp, game_service_message, GameServiceMessage, GameServiceMessageTypeE, Motion};
use crate::proto_gen::game_service_proto::game_service_message::Payload::{CharactersLoadRequest, CharactersUpdateRequest};


pub struct CmdCharacterUpdate;
impl CmdBase for CmdCharacterUpdate{

    fn exec(&self, character: &mut Player, msg: GameServiceMessage) -> Vec<u8> {
        match msg.payload.unwrap() {
            CharactersUpdateRequest(req) => Self::handle(req),
            _ => vec![],
        }
    }
}

impl CmdCharacterUpdate {
    pub fn new() -> Self {
        CmdCharacterUpdate
    }

    fn encode(resp: CharactersUpdateResp) -> Vec<u8> {
        let game_service_message = GameServiceMessage {
            id: GameServiceMessageTypeE::CharactersLoadRespE as i32,
            payload: Some(game_service_message::Payload::CharactersUpdateResponse(resp)),
        };

        let mut buffer = Vec::new();
        game_service_message.encode(&mut buffer).unwrap();
        buffer
    }

    fn handle(req: CharactersUpdateReq) -> Vec<u8> {
        println!("Received Character Load Request");
        for character in req.characters.iter() {
            dbg!(character);
        }
        let characters_update_resp = CharactersUpdateResp {
            result: true,
            characters: req.characters,
        };
        CmdCharacterUpdate::encode(characters_update_resp)
    }
}
