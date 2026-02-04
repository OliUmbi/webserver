use std::collections::VecDeque;
use std::sync::{mpsc, Arc};
use std::time::{Duration, Instant};

use crate::telemetry::telemetry::{Telemetry, TelemetryEvent};

const MAX_EVENT_HISTORY: usize = 1024;
const MAX_REQUESTS_HISTORY: usize = 256;

pub struct Tui {
    event_receiver: mpsc::Receiver<TelemetryEvent>,
    tick: Instant,
    pub telemetry: Arc<Telemetry>,
    pub event_history: VecDeque<TelemetryEvent>,
    pub requests_history: VecDeque<u64>,
}

impl Tui {
    pub fn new(telemetry: Arc<Telemetry>, event_receiver: mpsc::Receiver<TelemetryEvent>) -> Self {
        Self {
            event_receiver,
            tick: Instant::now(),
            telemetry,
            event_history: VecDeque::with_capacity(MAX_EVENT_HISTORY),
            requests_history: VecDeque::with_capacity(MAX_REQUESTS_HISTORY),
        }
    }

    pub fn update(&mut self) {
        if self.tick.elapsed() >= Duration::from_secs(1) {
            if self.requests_history.len() >= MAX_REQUESTS_HISTORY {
                self.requests_history.pop_back();
            }

            self.requests_history.push_front(self.telemetry.request_take() as u64);

            self.tick = Instant::now();
        }

        while let Ok(event) = self.event_receiver.try_recv() {
            if self.event_history.len() >= MAX_EVENT_HISTORY {
                self.event_history.pop_back();
            }

            self.event_history.push_front(event);
        }
    }
}
