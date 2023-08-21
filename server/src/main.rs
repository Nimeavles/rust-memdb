use crate::server::Server;

mod memdb;
mod parser;
mod query;
mod server;

pub use memdb::MemDb;
pub use parser::parse;
pub use query::Command;

fn main() {
    let server = Server::new("127.0.0.1", 4000);

    server.start_server();
}
