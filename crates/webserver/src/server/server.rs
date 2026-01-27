use crate::configuration::configuration::Configuration;
use crate::http::headers::Headers;
use crate::http::parser::body::parse_body;
use crate::http::parser::headers::parse_headers;
use crate::http::parser::request_head::parse_head;
use crate::http::parser::request_line::parse_request_line;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use crate::server::server_error::ServerError;
use std::io::{BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, mpsc};
use std::{fs, thread};
use crate::http::http_error::HttpError;

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

fn handle_connection(mut stream: TcpStream, configuration: &Configuration) {
    let response = handle_request(&stream, configuration);

    match stream.write_all(response.to_http().as_bytes()) {
        Ok(_) => {}
        Err(_) => println!("Failed to write response"), // todo review how to note this error
    }
}

fn handle_request(stream: &TcpStream, configuration: &Configuration) -> Response {
    let mut reader = BufReader::new(stream);

    let (raw_request_line, raw_headers, body_already_read) =
        match parse_head(&mut reader, configuration) {
            Ok(head) => head,
            Err(error) => return Response::from(HttpError::new(StatusCode::BadRequest, error)),
        };

    let request_line = match parse_request_line(raw_request_line) {
        Ok(request_line) => request_line,
        Err(error) => return Response::from(error),
    };

    let headers = match parse_headers(raw_headers) {
        Ok(headers) => headers,
        Err(error) => return Response::from(error),
    };

    // todo handle body
    let body = match parse_body(&mut reader, body_already_read, &headers) {
        Ok(body) => body,
        Err(error) => return Response::from(error),
    };

    println!("{:?}", request_line);
    println!("{:?}", headers);
    println!("{:?}", str::from_utf8(body.as_slice()).unwrap());

    let mut path = match request_line.url.raw.as_str() {
        "/" => "/index.html",
        value => value,
    };

    path = match fs::exists(format!(
        "C:/Users/olive/IdeaProjects/webserver/examples/demo/{}",
        path
    )) {
        Ok(exists) => {
            if exists {
                path
            } else {
                "/notfound.html"
            }
        }
        Err(_) => "/notfound.html",
    };

    let body = fs::read_to_string(format!(
        "C:/Users/olive/IdeaProjects/webserver/examples/demo/{}",
        path
    ))
    .unwrap();

    let mut response_headers = Headers::new();
    response_headers.add(
        "Content-Type".to_string(),
        if path.contains("html") {
            "text/html".to_string()
        } else {
            "text/css".to_string()
        },
    );
    response_headers.add("Content-Length".to_string(), body.len().to_string());

    Response::new(StatusCode::Ok, response_headers, body.to_string())
}
