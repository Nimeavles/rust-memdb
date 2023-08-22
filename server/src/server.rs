use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str, thread,
};

use crate::MemDb;
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
        //let client_address = client.peer_addr().unwrap();

        let mut bytes_from_client = [0u8; 1024];

        let mut memdb = MemDb::new();

        while let Ok(bytes) = client.read(&mut bytes_from_client) {
            if bytes < 1 {
                client.write(b"Error: Introduce a valid input!").unwrap();
                break;
            }

            let client_bytes_parsed = std::str::from_utf8(&bytes_from_client[..bytes])
                .unwrap()
                .trim();

            match memdb.execute(client_bytes_parsed) {
                Ok(success_exit) => {
                    let bytes_to_write = success_exit.as_bytes();
                    client.write(&bytes_to_write).unwrap();
                }
                Err(error_exit) => {
                    let bytes_to_write = error_exit.as_bytes();
                    client.write(&bytes_to_write).unwrap();
                }
            }
        }
    }
}
