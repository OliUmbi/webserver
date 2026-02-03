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
    Request { method: String, url: String },
    Info { message: String },
    Error { message: String },
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

    // todo handle error maybe
    pub fn event_request(&self, method: impl Into<String>, url: impl Into<String>) {
        let _ = self.event_sender.send(TelemetryEvent::Request {
            method: method.into(),
            url: url.into(),
        });
    }

    pub fn event_info(&self, message: impl Into<String>) {
        let _ = self.event_sender.send(TelemetryEvent::Info {
            message: message.into(),
        });
    }

    pub fn event_error(&self, message: impl Into<String>) {
        let _ = self.event_sender.send(TelemetryEvent::Error {
            message: message.into(),
        });
    }
}
