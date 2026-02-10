use std::sync::mpsc;

pub struct Logger {
    sender: mpsc::Sender<Log>,
}

impl Logger {
    pub fn new(sender: mpsc::Sender<Log>) -> Self {
        Self {
            sender
        }
    }

    pub fn success(&self, message: impl Into<String>) {
        self.sender
            .send(Log::new(LogType::Success, message))
            .unwrap();
    }

    pub fn success_details(&self, message: impl Into<String>, details: Vec<impl Into<String>>) {
        self.sender
            .send(Log::new_details(LogType::Success, message, details))
            .unwrap();
    }

    pub fn failed(&self, message: impl Into<String>) {
        self.sender
            .send(Log::new(LogType::Failed, message))
            .unwrap();
    }

    pub fn failed_details(&self, message: impl Into<String>, details: Vec<impl Into<String>>) {
        self.sender
            .send(Log::new_details(LogType::Failed, message, details))
            .unwrap();
    }

    pub fn information(&self, message: impl Into<String>) {
        self.sender
            .send(Log::new(LogType::Information, message))
            .unwrap();
    }

    pub fn information_details(&self, message: impl Into<String>, details: Vec<impl Into<String>>) {
        self.sender
            .send(Log::new_details(LogType::Information, message, details))
            .unwrap();
    }
}

pub struct Log {
    pub log_type: LogType,
    pub message: String,
    pub details: Vec<String>,
}

impl Log {
    pub fn new(log_type: LogType, message: impl Into<String>) -> Self {
        Self {
            log_type,
            message: message.into(),
            details: Vec::new(),
        }
    }

    pub fn new_details(
        log_type: LogType,
        message: impl Into<String>,
        details: Vec<impl Into<String>>,
    ) -> Self {
        Self {
            log_type,
            message: message.into(),
            details: details.into_iter().map(Into::into).collect(),
        }
    }
}

pub enum LogType {
    Success,
    Failed,
    Information,
}
