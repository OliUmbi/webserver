use std::thread::sleep;
use std::time::Duration;
use crate::configuration::parser::parse_configuration;
use crate::server::server::Server;

mod http;
mod server;
mod configuration;

fn main() {

    let configuration = match parse_configuration("./examples/simple/server.toml") {
        Ok(configuration) => {configuration}
        Err(error) => panic!("{}", error.message)
    };

    println!("{:?}", configuration);

    let server = match Server::start(configuration) {
        Ok(server) => server,
        Err(error) => panic!("{}", error.message)
    };

    sleep(Duration::from_secs(5));

    server.shutdown()
}
