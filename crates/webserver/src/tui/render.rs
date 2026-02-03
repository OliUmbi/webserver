use crate::tui::tui::Tui;
use crate::tui::tui_error::TuiError;
use crossterm::event::{poll, Event, KeyCode};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Style};
use ratatui::widgets::{
    Block, Borders, Gauge, Paragraph, Sparkline,
};
use ratatui::{Frame, Terminal};
use std::io;
use std::io::Stdout;
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_millis(200);

pub fn render(tui: &mut Tui) -> Result<(), TuiError> {
    let mut terminal = setup().map_err(|_| TuiError::new("Failed to setup terminal"))?;

    loop {
        tui.update();

        // todo maybe move draw to a function
        terminal
            .draw(|frame| {
                let [area_head, area_body] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
                    .areas(frame.area());

                let [area_stats, area_requests] = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                    .areas(area_head);

                let [area_workers, area_connections] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                    .areas(area_stats);

                render_workers(frame, area_workers, tui);
                render_connections(frame, area_connections, tui);
                render_requests(frame, area_requests, tui);
                render_events(frame, area_body, tui);
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

fn render_workers(frame: &mut Frame, area: Rect, tui: &Tui) {
    let data = tui.telemetry.workers();

    let block = Block::default().title("Workers").borders(Borders::ALL);

    let workers = Gauge::default()
        .block(block)
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(data as f64 / 8.0)
        .label(format!("{}/8", data));

    frame.render_widget(workers, area);
}

fn render_connections(frame: &mut Frame, area: Rect, tui: &Tui) {
    let data = tui.telemetry.connections();

    let block = Block::default().title("Connections").borders(Borders::ALL);

    let connections = Gauge::default()
        .block(block)
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(data as f64 / 40.0)
        .label(format!("{}/1024", data));

    frame.render_widget(connections, area);
}

fn render_requests(frame: &mut Frame, area: Rect, tui: &Tui) {
    let requests = Sparkline::default()
        .block(Block::new().title("Requests").borders(Borders::ALL))
        .data(tui.requests_history.clone())
        .style(Style::default().fg(Color::Blue));

    frame.render_widget(requests, area);
}

fn render_events(frame: &mut Frame, area: Rect, tui: &Tui) {
    let log_block = Block::default().title("Event Log").borders(Borders::ALL);
    let log_lines: Vec<Line> = tui
        .event_history
        .iter()
        .map(|e| Line::from(e.clone()))
        .collect();

    let height = area.height as usize;
    let total_events = tui.event_history.len();
    let scroll = if total_events > height {
        total_events - height
    } else {
        0
    };

    let events = Paragraph::new(log_lines)
        .block(log_block)
        .scroll((scroll as u16, 0));

    frame.render_widget(events, area);
}
