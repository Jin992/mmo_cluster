
use crate::proto_gen::game_service_proto::Character;

pub(crate) struct Player {
    character: Character,
}

impl Player {
    pub fn new() -> Self {
        Player{character: Character::default()}
    }
}

