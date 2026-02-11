use crate::http::request::Request;
use crate::test::configuration::Configuration;
use crate::test::log::Logger;
use crate::test::test::Test;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

const CONNECTIONS: usize = 32;
const REQUESTS: usize = 1024;

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
        let error_counter = Arc::new(AtomicUsize::new(0));
        let start = Instant::now();

        for _ in 0..CONNECTIONS {
            threads.push(stress_connection(configuration.clone(), logger.clone(), error_counter.clone()));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        let requests = CONNECTIONS * REQUESTS;
        let duration = start.elapsed().as_secs_f32();

        logger.success(format!("Requests: {}", requests));
        logger.success(format!("Time: {:.3}s", duration));
        logger.success(format!("Req/Sec {:.1}", (requests as f32) / duration));
        logger.success(format!("Errors: {}", error_counter.load(Ordering::Relaxed)));

        logger.information("Finished stress test");
    }
}

fn stress_connection(configuration: Arc<Configuration>, logger: Arc<Logger>, error_counter: Arc<AtomicUsize>) -> thread::JoinHandle<()> {
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
                        error_counter.fetch_add(1, Ordering::Relaxed);
                        logger.failed(format!("Non 200 status code {}", response.status_code));
                    }
                }
                Err(error) => {
                    error_counter.fetch_add(1, Ordering::Relaxed);
                    logger.failed(error)
                }
            }
        }
    })
}
