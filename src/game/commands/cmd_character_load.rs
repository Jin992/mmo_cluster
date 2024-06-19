use crate::game::player::Player;
use crate::game::commands::cmd_base::CmdBase;
use crate::proto_gen::game_service_proto::{CharactersLoadReq, game_service_message, GameServiceMessage};
use crate::proto_gen::game_service_proto::CharactersLoadResp;
use crate::proto_gen::game_service_proto::game_service_message::Payload;

pub struct CmdCharacterLoad;

impl CmdBase for CmdCharacterLoad {


    fn exec(&self, character: &mut Player, msg: GameServiceMessage) -> Vec<u8> {

        match msg.payload.unwrap() {
            Payload::CharactersLoadRequest(load_req) => {
                let player_token = load_req.token;

            }
            _ => {

            }}

        todo!()
    }
}

impl CmdCharacterLoad {
    pub fn new() -> Self {
        CmdCharacterLoad
    }
}
