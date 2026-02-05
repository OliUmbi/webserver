use crate::configuration::parser::parse_configuration;
use crate::server::server::Server;
use crate::telemetry::telemetry::Telemetry;
use crate::tui::tui::Tui;
use std::env;
use std::sync::{mpsc, Arc};

mod configuration;
mod handler;
mod http;
mod parser;
mod routing;
mod server;
mod telemetry;
mod tui;
mod validation;

fn main() {
    let configuration_path = match env::var("CONFIGURATION_PATH") {
        Ok(configuration_path) => configuration_path,
        Err(_) => panic!("Missing environment variable: CONFIGURATION_PATH"),
    };

    let configuration = match parse_configuration(configuration_path) {
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

    if configuration.headless {
        while let Ok(event) = event_receiver.recv() {
            println!("{}", event.to_string());
        }
    } else {
        let mut tui = Tui::new(telemetry, event_receiver);

        match tui::render::render(&mut tui, &configuration) {
            Ok(_) => (),
            Err(error) => panic!("{}", error.message),
        };
    }

    server.shutdown();
}
