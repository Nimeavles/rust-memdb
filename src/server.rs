use std::net::{TcpListener, TcpStream};
pub struct Server {
    listener: TcpListener,
    address: String,
}

impl Server {
    pub fn new(host: &str, port: u32) -> Self {
        let address_to_connect = format!("{host}:{port}");
        Self {
            address: address_to_connect.clone(),
            listener: TcpListener::bind(address_to_connect).expect("Couldn't bind to the port!"),
        }
    }

    pub fn start_server(&self) -> std::io::Result<()> {
        for connection in self.listener.incoming() {
            Server::handle_client(&connection?);
        }
        Ok(())
    }

    fn handle_client(client: &TcpStream) {
        println!("{:?}", client);
    }
}
