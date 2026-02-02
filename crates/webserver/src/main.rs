use crate::configuration::parser::parse_configuration;
use crate::server::server::Server;
use crate::telemetry::telemetry::Telemetry;
use std::sync::{mpsc, Arc};
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
    //
    // let mut routes = Vec::new();
    // routes.push(Route {
    //     path: Path::Exact("/hello".to_string()),
    //     action: Action::Redirect {
    //         to: "index.html".to_string(),
    //         code: StatusCode::TemporaryRedirect
    //     }
    // });
    // routes.push(Route {
    //     path: Path::Prefix("/".to_string()),
    //     action: Action::Fixed {
    //         root: PathBuf::from("./examples/demo/"),
    //         fallback: Some(PathBuf::from("./notfound.html"))
    //     }
    // });
    //
    // let conf = Configuration {
    //     server: configuration::server::Server::default(),
    //     routes
    // };
    //
    // println!("{}", toml::to_string(&conf).unwrap());

    let configuration = match parse_configuration("./examples/simple/server.toml") {
        Ok(configuration) => {configuration}
        Err(error) => panic!("{}", error.message)
    };

    println!("{:?}", configuration);

    let configuration = Arc::new(configuration);

    let (event_sender, event_receiver) = mpsc::channel();

    let telemetry = Arc::new(Telemetry::new(event_sender));

    let server = match Server::start(configuration.clone(), telemetry.clone()) {
        Ok(server) => server,
        Err(error) => panic!("{}", error.message)
    };

    let mut tui = Tui::new(telemetry, event_receiver);

    tui::render::render(&mut tui).unwrap();

    server.shutdown();

}
