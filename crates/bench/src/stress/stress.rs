use crate::http::request::Request;
use crate::test::configuration::Configuration;
use crate::test::log::Logger;
use crate::test::test::Test;
use std::sync::Arc;
use std::thread;

const CONNECTIONS: usize = 32;
const REQUESTS: usize = 128;

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

    fn run(&self, configuration: Arc<Configuration>, logger: Arc<Logger>) {
        logger.information("Starting stress test");

        let mut threads = Vec::with_capacity(CONNECTIONS);

        for _ in 0..CONNECTIONS {
            threads.push(stress_connection(configuration.clone(), logger.clone()));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        // todo duration, errors, etc.

        logger.information("Finished stress test");
    }
}

fn stress_connection(configuration: Arc<Configuration>, logger: Arc<Logger>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..REQUESTS {
            let request = Request::Structured {
                method: "GET",
                path: "/index.html",
                version: "HTTP/1.1",
                headers: vec!["Host: localhost"],
                body: "",
            };

            let response = request.send(configuration.address());

            match response {
                Ok(response) => {
                    if response.status_code != 200 {
                        logger.failed(format!("Non 200 status code {}", response.status_code));
                    }
                }
                Err(error) => logger.failed(error)
            }
        }
    })
}
