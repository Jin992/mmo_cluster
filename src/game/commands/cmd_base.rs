use crate::game::player::Player;
pub trait CmdBase: Send {
    fn exec(&self, character: &mut Player) -> Vec<u8>;
}