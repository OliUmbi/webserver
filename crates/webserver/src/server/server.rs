use crate::configuration::configuration::Configuration;
use crate::http::response::Response;
use crate::server::connection::Connection;
use crate::server::server_error::ServerError;
use crate::{handler, parser, routing};
use std::io::Write;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;

pub struct Server {
    running: Arc<AtomicBool>,
    acceptor: Option<thread::JoinHandle<()>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl Server {
    pub fn start(configuration: Configuration) -> Result<Self, ServerError> {
        // config
        let configuration = Arc::new(configuration);

        // running flag
        let running = Arc::new(AtomicBool::new(true));

        // setup channel
        let (sender, receiver) = mpsc::sync_channel(configuration.server.connections);
        let receiver = Arc::new(Mutex::new(receiver));

        // start listen
        let listener =
            match TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], configuration.server.port))) {
                Ok(listener) => listener,
                Err(error) => {
                    return Err(ServerError::new(format!(
                        "Failed to bind port: {}, error: {}",
                        configuration.server.port, error
                    )));
                }
            };

        // start acceptor
        let running_acceptor = Arc::clone(&running);
        let acceptor = thread::spawn(move || {
            println!("Acceptor started");
            for stream in listener.incoming() {
                if !running_acceptor.load(Ordering::Acquire) {
                    println!("Acceptor shutdown");
                    break;
                }

                let stream = match stream {
                    Ok(stream) => stream,
                    Err(_) => {
                        println!("Stream connection failed"); // todo investigate what to do if connection failed
                        continue;
                    }
                };

                if sender.send(stream).is_err() {
                    println!("Acceptor shutdown");
                    break;
                }
            }
        });

        // workers
        let mut workers = Vec::with_capacity(configuration.server.threads);
        for id in 0..configuration.server.threads {
            let receiver = Arc::clone(&receiver);
            let running = Arc::clone(&running);
            let configuration = Arc::clone(&configuration);

            workers.push(thread::spawn(move || {
                println!("Worker-{} started", id);
                while running.load(Ordering::Acquire) {
                    match receiver.lock().unwrap().recv() {
                        // todo think about poisoned mutex
                        Ok(stream) => handle_connection(stream, &configuration),
                        Err(_) => {
                            println!("Worker-{} shutdown", id);
                            break;
                        }
                    };
                }
            }));
        }

        Ok(Self {
            running,
            acceptor: Some(acceptor),
            workers,
        })
    }

    pub fn shutdown(mut self) {
        self.running.store(false, Ordering::Release);

        if let Some(acceptor) = self.acceptor.take() {
            let _ = acceptor.join();
        }

        for worker in self.workers {
            let _ = worker.join();
        }
    }
}

fn handle_connection(stream: TcpStream, configuration: &Configuration) {
    let mut connection = match Connection::new(stream) {
        Ok(connection) => connection,
        Err(error) => {
            println!("Connection construction failed: {}", error.message);
            return;
        }
    };

    let response = handle_request(&mut connection, configuration);

    match connection.write(response) {
        Ok(_) => {}
        Err(error) => println!("Connection construction failed: {}", error.message),
    }
}

fn handle_request(connection: &mut Connection, configuration: &Configuration) -> Response {
    // todo metadata (ip, time)

    let request = match parser::request::parse(connection, &configuration) {
        Ok(request) => request,
        Err(error) => return Response::from(error), // todo impl
    };

    let route = match routing::router::resolve(&request, &configuration) {
        Ok(route) => route,
        Err(error) => return Response::from(error), // todo impl
    };

    let response = match handler::route::handle(&request, &route, &configuration) {
        Ok(response) => response,
        Err(error) => return Response::from(error), // todo impl
    };

    response
}
