mod proto_gen {
    pub mod game_service_proto;
}
use protobuf::Message;

mod game {
    pub mod commands {
        pub mod cmd_base;
        pub mod cmd_character_load;
        pub mod cmd_character_update;
    }
    pub mod player;
}

mod network {
    pub mod server;
    pub mod session;
}


use crate::network::server::Server;

const SERVICE_IP_ADDR:&str = "127.0.0.1";
const SERVICE_PORT:&str = "6379";


fn main() {
    let mut server = Server::new(String::from(SERVICE_IP_ADDR), String::from(SERVICE_PORT));
    server.start();
}
