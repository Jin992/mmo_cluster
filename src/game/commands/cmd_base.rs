use crate::game::player::Player;
use crate::proto_gen::game_service_proto::GameServiceMessage;

pub trait CmdBase: Send {
    fn exec(&self, character: &mut Player, msg: GameServiceMessage) -> Vec<u8>;
}