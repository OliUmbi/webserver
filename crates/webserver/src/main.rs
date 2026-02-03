use crate::configuration::parser::parse_configuration;
use crate::server::server::Server;
use crate::telemetry::telemetry::Telemetry;
use std::sync::{mpsc, Arc};
use crate::configuration::external::raw_action::RawAction;
use crate::configuration::external::raw_configuration::RawConfiguration;
use crate::configuration::external::raw_path::RawPath;
use crate::configuration::external::raw_route::RawRoute;
use crate::configuration::external::raw_server::RawServer;
use crate::http::status_code::StatusCode;
use crate::tui::tui::Tui;

mod http;
mod server;
mod configuration;
mod routing;
mod handler;
mod parser;
mod tui;
mod telemetry;

fn main() {

    // let mut routes = Vec::new();
    // routes.push(RawRoute {
    //     path: RawPath::Exact("/hello".to_string()),
    //     action: RawAction::Redirect {
    //         location: "index.html".to_string(),
    //         status_code: StatusCode::TemporaryRedirect.code() as usize
    //     }
    // });
    // routes.push(RawRoute {
    //     path: RawPath::Prefix("/".to_string()),
    //     action: RawAction::Fixed {
    //         root: "./examples/demo/".to_string(),
    //         fallback: Some("./notfound.html".to_string())
    //     }
    // });
    //
    // let conf = RawConfiguration {
    //     server: RawServer::default(),
    //     routes
    // };
    //
    // println!("{}", toml::to_string(&conf).unwrap());

    let configuration = match parse_configuration("./examples/simple/server.toml") {
        Ok(configuration) => {configuration}
        Err(error) => panic!("{}", error.message)
    };

    let configuration = Arc::new(configuration);

    let (event_sender, event_receiver) = mpsc::channel();

    let telemetry = Arc::new(Telemetry::new(event_sender));

    let server = match Server::start(configuration.clone(), telemetry.clone()) {
        Ok(server) => server,
        Err(error) => panic!("{}", error.message)
    };

    let mut tui = Tui::new(telemetry, event_receiver);

    match tui::render::render(&mut tui) {
        Ok(_) => (),
        Err(error) => panic!("{}", error.message)
    };

    server.shutdown();

}
