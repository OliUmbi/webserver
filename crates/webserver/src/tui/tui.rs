use std::io;
use std::sync::Arc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crossterm::event::{poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::{event, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::telemetry::telemetry::Telemetry;

struct App {
    rps: u64,
    tick: Instant,
}

pub fn run(telemetry: Arc<Telemetry>) {

    enable_raw_mode().unwrap();
    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend).unwrap();

    let tick_rate = Duration::from_millis(100);
    let mut app = App {
        rps: 0,
        tick: Instant::now(),
    };

    // 2. Main Render Loop
    loop {
        // todo move
        if app.tick.elapsed() >= Duration::from_secs(1) {
            app.rps = telemetry.request_take() as u64;
            app.tick = Instant::now();
        }

        let telemetry = telemetry.clone();
        terminal.draw(|screen| {
            let size = screen.area();

            let block = Block::default()
                .title("Ratatui + Crossterm")
                .borders(Borders::ALL);

            let text = vec![
                Line::from(format!("Active workers: {}", telemetry.workers())),
                Line::from(format!("Queued connections: {}", telemetry.connections())),
                Line::from(format!("Requests / sec: {}", app.rps)),
                Line::from(""),
                Line::from("q = quit"),
            ];

            let paragraph = Paragraph::new(text).block(block);

            screen.render_widget(paragraph, size);
        }).unwrap();

        // 3. Event Handling
        if poll(tick_rate).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
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

}