use crate::http::status_code::StatusCode;
use chrono::{DateTime, SecondsFormat, Utc};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc;

#[derive(Debug)]
pub struct Telemetry {
    workers: AtomicUsize,
    connections: AtomicUsize,
    requests: AtomicUsize,
    event_sender: mpsc::Sender<TelemetryEvent>,
}

#[derive(Debug)]
pub enum TelemetryEvent {
    Request {
        time: DateTime<Utc>,
        method: String,
        url: String,
    },
    Info {
        time: DateTime<Utc>,
        message: String,
    },
    Error {
        time: DateTime<Utc>,
        message: String,
        status_code: Option<u16>,
    },
}

impl Telemetry {
    pub fn new(event_sender: mpsc::Sender<TelemetryEvent>) -> Self {
        Self {
            workers: AtomicUsize::new(0),
            connections: AtomicUsize::new(0),
            requests: AtomicUsize::new(0),
            event_sender,
        }
    }

    pub fn connections(&self) -> usize {
        self.connections.load(Ordering::Acquire)
    }

    pub fn connection_start(&self) {
        self.connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn connection_end(&self) {
        self.connections.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn workers(&self) -> usize {
        self.workers.load(Ordering::Acquire)
    }

    pub fn worker_start(&self) {
        self.workers.fetch_add(1, Ordering::Relaxed);
    }

    pub fn worker_end(&self) {
        self.workers.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn request_add(&self) {
        self.requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn request_take(&self) -> usize {
        self.requests.swap(1, Ordering::Relaxed)
    }

    pub fn event_request(&self, method: impl Into<String>, url: impl Into<String>) {
        let _ = self.event_sender.send(TelemetryEvent::Request {
            time: Utc::now(),
            method: method.into(),
            url: url.into(),
        });
    }

    pub fn event_info(&self, message: impl Into<String>) {
        let _ = self.event_sender.send(TelemetryEvent::Info {
            time: Utc::now(),
            message: message.into(),
        });
    }

    pub fn event_error(&self, message: impl Into<String>, status_code: Option<StatusCode>) {
        let _ = self.event_sender.send(TelemetryEvent::Error {
            time: Utc::now(),
            message: message.into(),
            status_code: status_code.map(|status_code| status_code.code()),
        });
    }
}

impl TelemetryEvent {
    pub fn to_string(&self) -> String {
        match self {
            TelemetryEvent::Request { time, method, url } => format!("{} [REQUEST] {} {}", time.to_rfc3339_opts(SecondsFormat::Secs, true), method, url),
            TelemetryEvent::Info { time, message } => format!("{} [INFO   ] {}", time.to_rfc3339_opts(SecondsFormat::Secs, true), message),
            TelemetryEvent::Error {
                time,
                message,
                status_code,
            } => format!(
                "{} [ERROR  ] {}{}",
                time.to_rfc3339_opts(SecondsFormat::Secs, true),
                status_code.map_or_else(|| String::new(), |status_code| format!("{} ", status_code)),
                message,
            ),
        }
    }
}
