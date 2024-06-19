use crate::game::player::Player;
use crate::game::commands::cmd_base::CmdBase;
use crate::game::commands::cmd_character_load::CmdCharacterLoad;


pub struct CmdCharacterUpdate;
impl CmdBase for CmdCharacterUpdate{

    fn exec(&self, character: &mut Player) -> Vec<u8> {
        todo!()
    }
}

impl CmdCharacterUpdate {
    pub fn new() -> Self {
        CmdCharacterUpdate
    }
}
