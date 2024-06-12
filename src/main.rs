mod generated {
    pub mod mmo_proto;
}

mod game {
    pub mod session;
    mod char {
        pub mod character;
        pub mod motion;
        pub mod vector_xyz;
    }
    mod commands {
        pub mod cmd_base;
        pub mod cmd_character_load;
        pub mod cmd_character_update;
    }
}

mod network {
    pub mod server;
}


use crate::network::server::Server;

const SERVICE_IP_ADDR:&str = "127.0.0.1";
const SERVICE_PORT:&str = "6379";


fn main() {
    let a1 = generated::mmo_proto::NetworkMessage::default();
    let mut server = Server::new(String::from(SERVICE_IP_ADDR), String::from(SERVICE_PORT));
    server.start();
}
