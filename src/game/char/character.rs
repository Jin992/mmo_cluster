use crate::game::char::motion::Motion;

#[derive(Default)]
pub struct Character {
    id: String,
    instance_id: String,
    map_id: String,
    motion: Motion,
}

impl Character {
    pub fn new() -> Self {
        Default::default()
    }
}