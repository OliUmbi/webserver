use crate::configuration::internal::configuration::Configuration;
use crate::telemetry::telemetry::TelemetryEvent;
use crate::tui::tui::Tui;
use crate::tui::tui_error::TuiError;
use crossterm::event::{poll, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::ListDirection::BottomToTop;
use ratatui::widgets::{
    Block, Borders, Gauge, List, ListItem, Paragraph, Sparkline,
};
use ratatui::{Frame, Terminal};
use std::io;
use std::io::Stdout;
use std::sync::Arc;
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_millis(200);

pub fn render(tui: &mut Tui, configuration: &Arc<Configuration>) -> Result<(), TuiError> {
    let mut terminal = setup().map_err(|_| TuiError::new("Failed to setup terminal"))?;

    loop {
        tui.update();

        terminal
            .draw(|frame| {
                let [area_head, area_body, area_foot] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(
                        [
                            Constraint::Length(10),
                            Constraint::Min(0),
                            Constraint::Length(3),
                        ]
                            .as_ref(),
                    )
                    .areas(frame.area());

                let [area_stats, area_requests] = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                    .areas(area_head);

                let [area_workers, area_connections] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .areas(area_stats);

                render_workers(frame, area_workers, tui, configuration);
                render_connections(frame, area_connections, tui, configuration);
                render_requests(frame, area_requests, tui);
                render_events(frame, area_body, tui);
                render_commands(frame, area_foot)
            })
            .map_err(|_| TuiError::new("Failed to draw terminal"))?;

        if poll(TICK_RATE).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    restore(terminal).map_err(|_| TuiError::new("Failed to restore terminal"))
}

fn setup() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

fn render_workers(frame: &mut Frame, area: Rect, tui: &Tui, configuration: &Arc<Configuration>) {
    let data = tui.telemetry.workers();

    let block = Block::default().title("Workers").borders(Borders::ALL);

    let workers = Gauge::default()
        .block(block)
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(data as f64 / configuration.server.threads as f64)
        .label(format!("{}/{}", data, configuration.server.threads));

    frame.render_widget(workers, area);
}

fn render_connections(
    frame: &mut Frame,
    area: Rect,
    tui: &Tui,
    configuration: &Arc<Configuration>,
) {
    let data = tui.telemetry.connections();

    let block = Block::default().title("Connections").borders(Borders::ALL);

    let connections = Gauge::default()
        .block(block)
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(data as f64 / configuration.server.connections as f64)
        .label(format!("{}/{}", data, configuration.server.connections));

    frame.render_widget(connections, area);
}

fn render_requests(frame: &mut Frame, area: Rect, tui: &Tui) {
    let block = Block::new().title("Requests").borders(Borders::ALL);

    let requests = Sparkline::default()
        .block(block)
        .data(tui.requests_history.clone())
        .style(Style::default().fg(Color::Blue));

    frame.render_widget(requests, area);
}

fn render_events(frame: &mut Frame, area: Rect, tui: &Tui) {
    let block = Block::default().title("Events").borders(Borders::ALL);

    let items: Vec<ListItem> = tui
        .event_history
        .iter()
        .map(|event| {
            let style = match event {
                TelemetryEvent::Request { .. } => Style::default().fg(Color::Blue),
                TelemetryEvent::Info { .. } => Style::default().fg(Color::Yellow),
                TelemetryEvent::Error { .. } => Style::default().fg(Color::Red),
            };

            ListItem::new(event.to_string()).style(style)
        })
        .collect();

    let events = List::new(items).block(block).direction(BottomToTop);

    frame.render_widget(events, area)
}

fn render_commands(frame: &mut Frame, area: Rect) {
    let block = Block::default().title("Commands").borders(Borders::ALL);

    let text = Text::raw("[Q] Quit");

    let commands = Paragraph::new(text).block(block);

    frame.render_widget(commands, area)
}
