use crate::game::char::character::Character;
use crate::game::commands::cmd_base::{CmdBase, CommandId};
use crate::game::commands::cmd_character_load::CmdCharacterLoad;


pub struct CmdCharacterUpdate {
    id: CommandId,
}

impl Default for CmdCharacterUpdate {
    fn default() -> Self {
        CmdCharacterUpdate {
            id: CommandId::CmdCharacterUpdate,
        }
    }
}

impl CmdBase for CmdCharacterUpdate{
    fn id(&self) -> CommandId {
        self.id.clone()
    }

    fn exec(&self, character: &mut Character) {
        todo!()
    }
}

impl CmdCharacterUpdate {
    pub fn new() -> Self {
        Default::default()
    }
}
