use std::io::{self, Stdout};
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};
use crossterm::{execute, event::{poll, Event, KeyCode}, event};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::widgets::{Block, Borders, Paragraph, Chart, Dataset, Axis};
use ratatui::layout::{Layout, Constraint, Direction, Rect};
use ratatui::style::{Style, Color};
use ratatui::text::{Line, Span};
use ratatui::Terminal;

use crate::telemetry::telemetry::{Telemetry, TelemetryEvent};

pub struct App {
    rps_history: Vec<u64>,
    events: Vec<String>,
    tick: Instant,
}

impl App {
    pub fn new() -> Self {
        Self {
            rps_history: Vec::with_capacity(60),
            events: Vec::new(),
            tick: Instant::now(),
        }
    }

    pub fn push_event(&mut self, msg: String) {
        self.events.push(msg);
        if self.events.len() > 50 {
            self.events.remove(0);
        }
    }

    pub fn push_rps(&mut self, rps: u64) {
        self.rps_history.push(rps);
        if self.rps_history.len() > 60 {
            self.rps_history.remove(0);
        }
    }
}

pub fn run(telemetry: Arc<Telemetry>, event_receiver: mpsc::Receiver<TelemetryEvent>) {
    let mut terminal = setup_terminal().unwrap();
    let mut app = App::new();
    let tick_rate = Duration::from_millis(200);

    loop {
        // Update RPS once per second
        if app.tick.elapsed() >= Duration::from_secs(1) {
            let rps = telemetry.request_take() as u64;
            app.push_rps(rps);
            app.tick = Instant::now();
        }

        // Drain telemetry events
        while let Ok(event) = event_receiver.try_recv() {
            app.push_event(format!("{:?}", event));
        }

        terminal.draw(|f| {
            let size = f.size();

            // Vertical split: top panel (stats + chart) / bottom panel (event log)
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
                .split(size);

            // Top panel horizontal split
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                .split(chunks[0]);

            // Left stats panel
            let stats_block = Block::default().title("Stats").borders(Borders::ALL);
            let stats_text = vec![
                Line::from(format!("Workers: {}", telemetry.workers())),
                Line::from(format!("Queued: {}", telemetry.connections())),
            ];
            let stats_paragraph = Paragraph::new(stats_text).block(stats_block);
            f.render_widget(stats_paragraph, top_chunks[0]);

            let max_rps = app.rps_history.iter().copied().max().unwrap_or(1) as f64;

            // Y-axis labels (e.g., 0, max/2, max)
            let y_labels = vec![
                Span::styled("0", Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", (max_rps / 2.0).round()), Style::default().fg(Color::Gray)),
                Span::styled(format!("{}", max_rps.round()), Style::default().fg(Color::Gray)),
            ];

            // Dataset for the line
            let rps_data: Vec<(f64, f64)> = app.rps_history.iter().enumerate()
                .map(|(i, v)| (i as f64, *v as f64))
                .collect();

            let dataset = Dataset::default()
                .name("RPS")
                .marker(ratatui::symbols::Marker::Dot)
                .style(Style::default().fg(Color::Cyan))
                .data(&rps_data);

            // Chart with Y-axis labels
            let chart = Chart::new(vec![dataset])
                .block(Block::default().title("Requests / sec").borders(Borders::ALL))
                .x_axis(Axis::default().bounds([0.0, 60.0]))
                .y_axis(Axis::default()
                    .bounds([0.0, max_rps])
                    .labels(y_labels)
                );

            f.render_widget(chart, top_chunks[1]);

            let rps = *app.rps_history.last().unwrap_or(&0);

            // Overlay a small paragraph at the top-right of the chart
            let label = Paragraph::new(Span::styled(
                format!("{} req/s", rps),
                Style::default().fg(Color::Yellow),
            ))
                .block(Block::default()) // optional border if you like
                .style(Style::default());

            // Example: position at top-right of the chart
            let label_rect = Rect {
                x: top_chunks[1].x + top_chunks[1].width - 12, // 12 = approx width of label
                y: top_chunks[1].y,
                width: 12,
                height: 1,
            };
            f.render_widget(label, label_rect);

            // Bottom log panel
            let log_block = Block::default().title("Event Log").borders(Borders::ALL);
            let log_lines: Vec<Line> = app.events.iter().map(|e| Line::from(e.clone())).collect();

            let log_height = chunks[1].height as usize; // visible lines
            let total_events = app.events.len();
            let scroll = if total_events > log_height { total_events - log_height } else { 0 };

            let log_paragraph = Paragraph::new(log_lines)
                .block(log_block)
                .scroll((scroll as u16, 0)); // vertical scroll
            f.render_widget(log_paragraph, chunks[1]);
        }).unwrap();

        // Non-blocking input
        if poll(tick_rate).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    restore_terminal(terminal).unwrap();
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(mut terminal: Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}
