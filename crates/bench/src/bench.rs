use crate::test::configuration::Configuration;
use crate::test::log::{Log, LogType, Logger};
use crate::test::test::Test;
use crossterm::event::{poll, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, execute};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Style, Text};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Clear, List, ListDirection, ListItem, Paragraph};
use ratatui::{Frame, Terminal};
use std::io::Stdout;
use std::sync::{mpsc, Arc};
use std::time::Duration;
use std::{io, thread};
use ratatui::style::Stylize;

const TICK_RATE: Duration = Duration::from_millis(200);

pub struct Bench {
    tests: Vec<Arc<dyn Test>>,
    running: bool,
    logger: Arc<Logger>,
    log_receiver: mpsc::Receiver<Log>,
    selected_test: usize,
    logs: Vec<Log>,
}

impl Bench {
    pub fn new(tests: Vec<Arc<dyn Test>>) -> Self {
        if tests.is_empty() {
            panic!("No tests specified")
        }

        let (log_sender, log_receiver) = mpsc::channel();

        Self {
            tests,
            running: true,
            logger: Arc::new(Logger::new(log_sender)),
            log_receiver,
            selected_test: 0,
            logs: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        let mut terminal = setup().unwrap();

        while self.running {
            self.handle_event();

            terminal.draw(|frame| self.render(frame)).unwrap();
        }

        restore(terminal).unwrap();
    }

    fn handle_event(&mut self) {
        if poll(TICK_RATE).unwrap() {
            if let Event::Key(key) = event::read().unwrap()
                && key.kind == KeyEventKind::Press
            {
                if key.code == KeyCode::Char('q') {
                    self.running = false
                }
                if key.code == KeyCode::Up && self.selected_test != 0 {
                    self.selected_test -= 1;
                }
                if key.code == KeyCode::Down && self.selected_test != self.tests.len() - 1 {
                    self.selected_test += 1;
                }
                if key.code == KeyCode::Enter {
                    self.start_test()
                }
            }
        }
    }

    fn start_test(&self) {
        let test = self.tests.get(self.selected_test).unwrap();

        let configuration = Arc::new(Configuration::new("127.0.0.1:80"));

        let test = test.clone();
        let logger = self.logger.clone();

        thread::spawn(move || test.run(configuration, logger));
    }

    fn render(&mut self, frame: &mut Frame) {
        let [area_body, area_foot] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
            .areas(frame.area());

        let [area_tests, area_logs] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(40), Constraint::Min(0)].as_ref())
            .areas(area_body);

        self.render_tests(frame, area_tests);
        self.render_logs(frame, area_logs);
        self.render_configuration(frame, area_body);
        self.render_commands(frame, area_foot);
    }

    fn render_tests(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().title("Tests").borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .tests
            .iter()
            .enumerate()
            .map(|(index, test)| {
                let mut style = Style::default();

                if index == self.selected_test {
                    style = style.fg(Color::Black).bg(Color::White);
                }

                ListItem::new(test.name()).style(style)
            })
            .collect();

        let tests = List::new(items).block(block);

        frame.render_widget(tests, area)
    }

    fn render_logs(&mut self, frame: &mut Frame, area: Rect) {
        while let Ok(log) = self.log_receiver.try_recv() {
            self.logs.push(log);
        }

        let block = Block::default().title("Logs").borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .logs
            .iter()
            .rev()
            .map(|(log)| {
                let style = match log.log_type {
                    LogType::Success => Style::default().fg(Color::Green),
                    LogType::Failed => Style::default().fg(Color::Red),
                    LogType::Information => Style::default().fg(Color::White),
                };

                let mut lines = Vec::new();
                lines.push(Line::from(log.message.clone()));
                log.details
                    .iter()
                    .for_each(|detail| lines.push(Line::from(detail.clone())));

                ListItem::new(Text::from(lines)).style(style)
            })
            .collect();

        let tests = List::new(items)
            .direction(ListDirection::BottomToTop)
            .block(block);

        frame.render_widget(tests, area)
    }

    fn render_configuration(&self, frame: &mut Frame, area: Rect) {
        let area = area.centered(Constraint::Length(100), Constraint::Length(5));

        let block = Block::default().title("Configuration").borders(Borders::ALL).on_dark_gray();
        let text = Text::raw("Address");
        let commands = Paragraph::new(text).block(block);

        frame.render_widget(Clear, area);
        frame.render_widget(commands, area)
    }

    fn render_commands(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().title("Commands").borders(Borders::ALL);
        let text = Text::raw("[Q] Quit | [↑] Up | [↓] Down | [Enter] Run");
        let commands = Paragraph::new(text).block(block);

        frame.render_widget(commands, area)
    }
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
