use crate::server::Server;

mod parser;
mod query;
mod server;

pub use parser::parse;
pub use query::Command;

fn main() {
    let server = Server::new("127.0.0.1", 3000);

    server.start_server();
}
