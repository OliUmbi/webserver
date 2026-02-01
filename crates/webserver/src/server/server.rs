use crate::configuration::configuration::Configuration;
use crate::http::response::Response;
use crate::server::connection::Connection;
use crate::server::server_error::ServerError;
use crate::{handler, parser, routing};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;
use ratatui::run;
use crate::telemetry::telemetry::Telemetry;

pub struct Server {
    running: Arc<AtomicBool>,
    acceptor: Option<thread::JoinHandle<()>>,
    dispatcher: Option<thread::JoinHandle<()>>,
    workers: Vec<thread::JoinHandle<()>>,
}

impl Server {
    pub fn start(
        configuration: Arc<Configuration>,
        telemetry: Arc<Telemetry>,
    ) -> Result<Self, ServerError> {
        let running = Arc::new(AtomicBool::new(true));

        let (dispatch_sender, dispatch_receiver) = mpsc::sync_channel(configuration.server.connections);

        let mut worker_senders = Vec::with_capacity(configuration.server.threads);
        let mut worker_receivers = Vec::with_capacity(configuration.server.threads);
        for _ in 0..configuration.server.threads {
            let (worker_sender, worker_receiver) = mpsc::sync_channel::<TcpStream>(1);
            worker_senders.push(worker_sender);
            worker_receivers.push(worker_receiver);
        }

        let listener = Self::start_listener(&configuration)?;

        let acceptor = Self::start_acceptor(listener, running.clone(), dispatch_sender, telemetry.clone());

        let dispatcher = Self::start_dispatcher(running.clone(), dispatch_receiver, worker_senders);

        let workers = Self::start_workers(configuration.clone(), running.clone(), worker_receivers, telemetry.clone());

        Ok(Self {
            running,
            acceptor: Some(acceptor),
            dispatcher: Some(dispatcher),
            workers,
        })
    }

    pub fn shutdown(mut self) {
        self.running.store(false, Ordering::Release);

        if let Some(acceptor) = self.acceptor.take() {
            let _ = acceptor.join();
        }

        if let Some(dispatcher) = self.dispatcher.take() {
            let _ = dispatcher.join();
        }

        for worker in self.workers {
            let _ = worker.join();
        }
    }

    fn start_listener(configuration: &Arc<Configuration>) -> Result<TcpListener, ServerError> {
        TcpListener::bind(SocketAddr::from(([0, 0, 0, 0], configuration.server.port)))
            .map_err(|error| ServerError::new(format!("Failed to bind port: {}, error: {}", configuration.server.port, error)))
    }

    fn start_acceptor(listener: TcpListener, running: Arc<AtomicBool>, dispatch_sender: mpsc::SyncSender<TcpStream>, telemetry: Arc<Telemetry>) -> thread::JoinHandle<()> {
        thread::spawn(move || {

            println!("Acceptor started");

            for stream in listener.incoming() {
                if !running.load(Ordering::Acquire) {

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

                // todo review
                telemetry.connection_start();

                if dispatch_sender.send(stream).is_err() {

                    println!("Acceptor shutdown");

                    break;
                }
            }
        })
    }

    fn start_dispatcher(running: Arc<AtomicBool>, dispatch_receiver: mpsc::Receiver<TcpStream>, worker_senders: Vec<mpsc::SyncSender<TcpStream>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut next = 0;

            while running.load(Ordering::Acquire) {
                let stream = match dispatch_receiver.recv() {
                    Ok(stream) => stream,
                    Err(_) => break,
                };

                // todo telemetry dispatcher_telemetry.connection_start();

                let worker_sender = &worker_senders[next];
                // todo error handling
                if worker_sender.send(stream).is_err() {
                    break;
                }

                next = (next + 1) % worker_senders.len();
            }
        })
    }

    fn start_workers(configuration: Arc<Configuration>, running: Arc<AtomicBool>, worker_receivers: Vec<mpsc::Receiver<TcpStream>>, telemetry: Arc<Telemetry>) -> Vec<thread::JoinHandle<()>> {

        let mut workers = Vec::with_capacity(configuration.server.threads);

        for (id, worker_receiver) in worker_receivers.into_iter().enumerate() {
            let worker_running = Arc::clone(&running);
            let worker_configuration = Arc::clone(&configuration);
            let worker_telemetry = Arc::clone(&telemetry);

            workers.push(thread::spawn(move || {

                println!("Worker-{} started", id);

                while worker_running.load(Ordering::Acquire) {
                    match worker_receiver.recv() {
                        Ok(stream) => {
                            worker_telemetry.worker_start();

                            handle_connection(stream, &worker_configuration, &worker_telemetry);

                            worker_telemetry.worker_end();
                            worker_telemetry.connection_end();
                        },
                        Err(_) => {
                            println!("Worker-{} shutdown", id);
                            break;
                        }
                    };
                }
            }));
        }

        workers
    }
}

fn handle_connection(stream: TcpStream, configuration: &Configuration, telemetry: &Telemetry) {
    let mut connection = match Connection::new(stream) {
        Ok(connection) => connection,
        Err(error) => {
            println!("Connection construction failed: {}", error.message);
            return;
        }
    };

    telemetry.request_add();

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
