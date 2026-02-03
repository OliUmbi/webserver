use crate::configuration::internal::configuration::Configuration;
use crate::http::response::Response;
use crate::server::connection::Connection;
use crate::server::server_error::ServerError;
use crate::telemetry::telemetry::Telemetry;
use crate::{handler, parser, routing};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

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

        let (dispatch_sender, dispatch_receiver) =
            mpsc::sync_channel(configuration.server.connections);

        let mut worker_senders = Vec::with_capacity(configuration.server.threads);
        let mut worker_receivers = Vec::with_capacity(configuration.server.threads);
        for _ in 0..configuration.server.threads {
            let (worker_sender, worker_receiver) =
                mpsc::sync_channel::<TcpStream>(configuration.server.connections);
            worker_senders.push(worker_sender);
            worker_receivers.push(worker_receiver);
        }

        let listener = Self::start_listener(&configuration)?;

        let acceptor = Self::start_acceptor(
            listener,
            running.clone(),
            dispatch_sender,
            telemetry.clone(),
        );

        let dispatcher = Self::start_dispatcher(
            running.clone(),
            dispatch_receiver,
            worker_senders,
            telemetry.clone(),
        );

        let workers = Self::start_workers(
            configuration.clone(),
            running.clone(),
            worker_receivers,
            telemetry.clone(),
        );

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
        TcpListener::bind(SocketAddr::from((
            [0, 0, 0, 0],
            configuration.server.port as u16,
        )))
        .map_err(|error| {
            ServerError::new(format!(
                "Failed to bind port: {}, error: {}",
                configuration.server.port, error
            ))
        })
    }

    fn start_acceptor(
        listener: TcpListener,
        running: Arc<AtomicBool>,
        dispatch_sender: mpsc::SyncSender<TcpStream>,
        telemetry: Arc<Telemetry>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            telemetry.event_info("Acceptor started");

            for stream in listener.incoming() {
                if !running.load(Ordering::Acquire) {
                    telemetry.event_info("Acceptor shutdown");
                    break;
                }

                let stream = match stream {
                    Ok(stream) => stream,
                    Err(_) => {
                        telemetry.event_error("Stream connection failed");
                        continue;
                    }
                };

                // todo review
                telemetry.connection_start();

                if dispatch_sender.send(stream).is_err() {
                    telemetry.event_info("Acceptor shutdown");
                    break;
                }
            }
        })
    }

    fn start_dispatcher(
        running: Arc<AtomicBool>,
        dispatch_receiver: mpsc::Receiver<TcpStream>,
        worker_senders: Vec<mpsc::SyncSender<TcpStream>>,
        telemetry: Arc<Telemetry>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            telemetry.event_info("Dispatcher started");
            let mut next = 0;

            while running.load(Ordering::Acquire) {
                let stream = match dispatch_receiver.recv() {
                    Ok(stream) => stream,
                    Err(_) => {
                        telemetry.event_info("Dispatcher shutdown");
                        break;
                    }
                };

                let worker_sender = &worker_senders[next];

                if worker_sender.send(stream).is_err() {
                    telemetry.event_info("Dispatcher shutdown");
                    break;
                }

                next = (next + 1) % worker_senders.len();
            }
        })
    }

    fn start_workers(
        configuration: Arc<Configuration>,
        running: Arc<AtomicBool>,
        worker_receivers: Vec<mpsc::Receiver<TcpStream>>,
        telemetry: Arc<Telemetry>,
    ) -> Vec<thread::JoinHandle<()>> {
        let mut workers = Vec::with_capacity(configuration.server.threads);

        for (id, worker_receiver) in worker_receivers.into_iter().enumerate() {
            workers.push(Self::start_worker(
                id,
                configuration.clone(),
                running.clone(),
                worker_receiver,
                telemetry.clone(),
            ));
        }

        workers
    }

    fn start_worker(
        id: usize,
        configuration: Arc<Configuration>,
        running: Arc<AtomicBool>,
        worker_receiver: mpsc::Receiver<TcpStream>,
        telemetry: Arc<Telemetry>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            telemetry.event_info(format!("Worker-{} started", id));

            while running.load(Ordering::Acquire) {
                match worker_receiver.recv() {
                    Ok(stream) => {
                        telemetry.worker_start();

                        handle_connection(stream, &configuration, &telemetry);

                        telemetry.worker_end();
                        telemetry.connection_end();
                    }
                    Err(_) => {
                        telemetry.event_info(format!("Worker-{} shutdown", id));
                        break;
                    }
                };
            }
        })
    }
}

fn handle_connection(stream: TcpStream, configuration: &Configuration, telemetry: &Telemetry) {
    let mut connection = match Connection::new(stream) {
        Ok(connection) => connection,
        Err(error) => {
            telemetry.event_error(format!("Connection construction failed: {}", error.message));
            return;
        }
    };

    telemetry.request_add();

    let response = handle_request(&mut connection, configuration, telemetry);

    match connection.write_response(response) {
        Ok(_) => {}
        Err(error) => telemetry.event_error(format!("Connection write failed: {}", error.message)),
    }
}

fn handle_request(
    connection: &mut Connection,
    configuration: &Configuration,
    telemetry: &Telemetry,
) -> Response {
    // todo metadata (ip, time)

    let mut request = match parser::request::parse(connection, &configuration) {
        Ok(request) => request,
        Err(error) => return Response::from(error), // todo impl
    };

    telemetry.event_request(
        request.request_line.method.as_str(),
        &request.request_line.url.raw,
    );

    let route = match routing::router::resolve(&request, &configuration) {
        Ok(route) => route,
        Err(error) => return Response::from(error), // todo impl
    };

    let response = match handler::route::handle(&mut request, &route, connection, &configuration) {
        Ok(response) => response,
        Err(error) => return Response::from(error), // todo impl
    };

    response
}
