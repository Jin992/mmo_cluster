use crate::game::char::character::Character;
#[derive(Clone)]
pub enum CommandId {
    CmdCharacterLoad = 1,
    CmdCharacterUpdate = 2,
}
pub trait CmdBase {
    fn id(&self) -> CommandId;
    fn exec(&self, character: &mut Character);
}