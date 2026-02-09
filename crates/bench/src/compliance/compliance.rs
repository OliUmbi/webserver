use crate::http::request::Request;
use crate::test::configuration::Configuration;
use crate::test::log::Logger;
use crate::test::test::Test;
use std::sync::Arc;

pub struct Compliance {}

impl Compliance {
    pub fn new() -> Self {
        Self {}
    }
}

impl Test for Compliance {
    fn name(&self) -> String {
        "Compliance".to_string()
    }

    fn run(&self, configuration: Configuration, logger: Arc<Logger>) {
        for case in cases() {
            let response = case.request.send(configuration.address());

            match response {
                Ok(response) => {
                    if case.status.contains(&response.status_code) {
                        logger.success(case.name);
                    } else {
                        logger.failed(format!("{} invalid status code {} expected {:?}", case.name, response.status_code, case.status));
                    }
                }
                Err(error) => {
                    if case.status.len() == 0 {
                        logger.success(case.name);
                    } else {
                        logger.failed(error)
                    }
                },
            }
        }
    }
}

pub struct Case {
    pub name: &'static str,
    pub request: Request,
    pub status: Vec<usize>,
}

pub fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "Valid GET",
            request: Request::Structured {
                method: "GET",
                path: "/index.html",
                version: "HTTP/1.1",
                headers: vec!["Host: bench"],
                body: "",
            },
            status: vec![200]
        },
        Case {
            name: "Fragmented method",
            request: Request::Raw("G"),
            status: vec![400]
        },
        Case {
            name: "Url missing",
            request: Request::Raw("GET "),
            status: vec![400]
        },
        Case {
            name: "Version missing",
            request: Request::Raw("GET /index.html "),
            status: vec![400]
        },
        Case {
            name: "Fragmented version",
            request: Request::Raw("GET /index.html HTTP"),
            status: vec![400]
        },
        Case {
            name: "Unsupported version",
            request: Request::Raw("GET /index.html HTTP/1.0"),
            status: vec![400]
        },
        Case {
            name: "Fragmented request line 1",
            request: Request::Raw("GET /index.html HTTP/1.1\r"),
            status: vec![400]
        },
        Case {
            name: "Fragmented request line 2",
            request: Request::Raw("GET /index.html HTTP/1.1\r\n"),
            status: vec![400]
        },
        Case {
            name: "Fragmented header name",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHos"),
            status: vec![400]
        },
        Case {
            name: "Fragmented header value 1",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost:"),
            status: vec![400]
        },
        Case {
            name: "Fragmented header value 2",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: "),
            status: vec![400]
        },
        Case {
            name: "Fragmented header value 3",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench"),
            status: vec![400]
        },
        Case {
            name: "Fragmented header 1",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r"),
            status: vec![400]
        },
        Case {
            name: "Fragmented header 2",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\n"),
            status: vec![400]
        },
        Case {
            name: "Fragmented header 3",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\n\r"),
            status: vec![400]
        },
    ]
}
