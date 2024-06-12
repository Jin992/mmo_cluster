use crate::game::char::character::Character;
use crate::game::commands::cmd_base::CmdBase;
use crate::game::commands::cmd_base::CommandId;
pub struct CmdCharacterLoad {
    id: CommandId,
}

impl Default for CmdCharacterLoad {
    fn default() -> Self {
        CmdCharacterLoad {
            id: CommandId::CmdCharacterLoad,
        }
    }
}

impl CmdBase for CmdCharacterLoad {
    fn id(&self) -> CommandId {
        self.id.clone()
    }

    fn exec(&self, character: &mut Character) {
        todo!()
    }
}

impl CmdCharacterLoad {
    pub fn new() -> Self {
        Default::default()
    }
}
