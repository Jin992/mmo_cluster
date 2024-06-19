use crate::game::player::Player;
use crate::game::commands::cmd_base::CmdBase;
pub struct CmdCharacterLoad;

impl CmdBase for CmdCharacterLoad {

    fn exec(&self, character: &mut Player) -> Vec<u8> {
        todo!()
    }
}

impl CmdCharacterLoad {
    pub fn new() -> Self {
        CmdCharacterLoad
    }
}
