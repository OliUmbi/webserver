use crate::configuration::parser::parse_configuration;
use crate::server::server::Server;
use crate::telemetry::telemetry::Telemetry;
use crate::tui::tui::Tui;
use std::sync::{mpsc, Arc};

mod configuration;
mod handler;
mod http;
mod parser;
mod routing;
mod server;
mod telemetry;
mod tui;

fn main() {
    let configuration = match parse_configuration("./examples/simple/server.toml") {
        Ok(configuration) => configuration,
        Err(error) => panic!("{}", error.message),
    };

    let configuration = Arc::new(configuration);

    let (event_sender, event_receiver) = mpsc::channel();

    let telemetry = Arc::new(Telemetry::new(event_sender));

    let server = match Server::start(configuration.clone(), telemetry.clone()) {
        Ok(server) => server,
        Err(error) => panic!("{}", error.message),
    };

    let mut tui = Tui::new(telemetry, event_receiver);

    match tui::render::render(&mut tui) {
        Ok(_) => (),
        Err(error) => panic!("{}", error.message),
    };

    server.shutdown();
}
