use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};
pub struct Server {
    listener: TcpListener,
    address: String,
}

impl Server {
    pub fn new(host: &str, port: u16) -> Self {
        let address_to_connect = format!("{host}:{port}");
        Self {
            address: address_to_connect.clone(),
            listener: TcpListener::bind(address_to_connect).expect("Couldn't bind to the port!"),
        }
    }

    pub fn start_server(&self) {
        println!("[*] The server started at {}", self.address);
        for connection in self.listener.incoming() {
            /* TODO: thread pool*/
            thread::spawn(|| {
                Server::handle_client(&mut connection.unwrap());
            });
        }
    }

    fn handle_client(client: &mut TcpStream) {
        println!("[*] New Connection From: {}", client.peer_addr().unwrap());
        let mut bytes_from_client = [0u8; 1024];

        while let Ok(bytes) = client.read(&mut bytes_from_client) {
            if bytes < 1 {
                client.write(b"Error: Introduce a valid input!").unwrap();
                break;
            }

            let client_bytes_parsed = std::str::from_utf8(&bytes_from_client[..bytes])
                .unwrap()
                .trim();

            println!("{client_bytes_parsed}");
        }
    }
}
