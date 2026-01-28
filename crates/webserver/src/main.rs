use crate::configuration::parser::parse_configuration;
use crate::server::server::Server;

mod http;
mod server;
mod configuration;
mod routing;
mod handler;
pub mod parser;

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
