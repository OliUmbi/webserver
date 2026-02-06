use std::sync::Arc;
use std::thread;
use crate::http::request::Request;
use crate::test::configuration::Configuration;
use crate::test::log::Logger;
use crate::test::test::Test;

pub struct Stress {}

impl Stress {
    pub fn new() -> Self {
        Self {}
    }
}

impl Test for Stress {
    fn name(&self) -> String {
        "Stress".to_string()
    }

    fn run(&self, configuration: Configuration, logger: Logger) {

        let logger = Arc::new(logger);
        let configuration = Arc::new(configuration);

        logger.information("Starting stress test");

        let mut threads = Vec::with_capacity(configuration.custom_usize("connections"));

        for _ in 0..configuration.custom_usize("connections") {
            threads.push(stress_connection(configuration.clone(), logger.clone()));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        logger.information("Finished stress test");
    }
}

fn stress_connection(configuration: Arc<Configuration>, logger: Arc<Logger>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..configuration.custom_usize("requests") {
            let request = Request::Structured {
                address: configuration.address(),
                method: "GET",
                path: "/index.html",
                version: "HTTP/1.1",
                headers: vec!["Host: localhost"],
                body: "",
            };

            let mut response = request.send();

            if response.status_code() != 200 {
                logger.failed(format!("Non 200 status code {}", response.status_code()));
            }
        }
    })
}
