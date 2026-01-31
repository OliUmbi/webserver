use std::path::PathBuf;
use crate::configuration::action::Action;
use crate::configuration::configuration::Configuration;
use crate::configuration::parser::parse_configuration;
use crate::configuration::path::Path;
use crate::configuration::route::Route;
use crate::http::status_code::StatusCode;
use crate::server::server::Server;

mod http;
mod server;
mod configuration;
mod routing;
mod handler;
pub mod parser;

fn main() {

    let mut routes = Vec::new();
    routes.push(Route {
        path: Path::Exact("/hello".to_string()),
        action: Action::Redirect {
            to: "index.html".to_string(),
            code: StatusCode::TemporaryRedirect
        }
    });
    routes.push(Route {
        path: Path::Prefix("/".to_string()),
        action: Action::Fixed {
            root: PathBuf::from("./examples/demo/"),
            fallback: Some(PathBuf::from("./notfound.html"))
        }
    });

    let conf = Configuration {
        server: configuration::server::Server::default(),
        routes
    };

    println!("{}", toml::to_string(&conf).unwrap());

    let configuration = match parse_configuration("./examples/simple/server.toml") {
        Ok(configuration) => {configuration}
        Err(error) => panic!("{}", error.message)
    };

    println!("{:?}", configuration);

    let server = match Server::start(configuration) {
        Ok(server) => server,
        Err(error) => panic!("{}", error.message)
    };


    /* ------------

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    // 2. Main Render Loop
    loop {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title("Ratatui + Crossterm")
                .borders(Borders::ALL);
            f.render_widget(block, size);
        }).unwrap();

        // 3. Event Handling
        if let Event::Key(key) = event::read().unwrap() {
            if KeyCode::Char('q') == key.code {
                break;
            }
        }
    }

    // 4. Restore terminal on exit
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    ).unwrap();
    terminal.show_cursor().unwrap();

    ------------ */

    // sleep(Duration::from_secs(5));

    // server.shutdown()

    loop {

    }
}
