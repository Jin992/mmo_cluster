use tokio::net::TcpListener;
use crate::game::session::GameSession;

pub struct Server {
    ip_addr: String,
    port: String
}

impl Server {
    pub fn new(ip_addr: String, port: String) -> Self {
        Server{ip_addr, port}
    }
    #[tokio::main]
    pub async fn start(&mut self) {
        let _service_addr:String = [self.ip_addr.clone(), self.port.clone()].join(":");
        let listener = TcpListener::bind(_service_addr.clone()).await.unwrap();

        println!("Listening {}", _service_addr);
        loop {
            let (socket, _) = listener.accept().await.unwrap();

            println!("Accepted");
            tokio::spawn(async move {
                let mut session = GameSession::new(socket);
                session.start().await;
            });
        }
    }
}