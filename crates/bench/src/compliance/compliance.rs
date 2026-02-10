use crate::http::request::Request;
use crate::test::configuration::Configuration;
use crate::test::log::Logger;
use crate::test::test::Test;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

const THREADS: usize = 8;

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

    fn run(&self, configuration: Arc<Configuration>, logger: Arc<Logger>) {

        logger.information("Started compliance test");

        let mut cases_per_thread: Vec<Vec<Case>> = Vec::with_capacity(THREADS);
        for _ in 0..THREADS {
            cases_per_thread.push(Vec::new());
        }

        for (i, case) in cases().into_iter().enumerate() {
            cases_per_thread[i % THREADS].push(case);
        }

        let mut threads = Vec::with_capacity(THREADS);

        for cases in cases_per_thread {
            threads.push(run_case(cases, configuration.clone(), logger.clone()));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        logger.information("Finished compliance test");
    }
}

fn run_case(cases: Vec<Case>, configuration: Arc<Configuration>, logger: Arc<Logger>) -> JoinHandle<()> {
    thread::spawn(move || {
        for case in cases {
            let response = case.request.send(configuration.address());

            match response {
                Ok(response) => {
                    if case.status.contains(&response.status_code) {
                        logger.success(case.name);
                    } else {
                        logger.failed(format!("{}: invalid status code {} expected {:?}", case.name, response.status_code, case.status));
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
    })
}

pub struct Case {
    pub name: &'static str,
    pub request: Request,
    pub status: Vec<usize>,
}

pub fn cases() -> Vec<Case> {
    vec![
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
            name: "Invalid version",
            request: Request::Raw("GET /index.html HTTP/9.9"),
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
        Case {
            name: "Expect header",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nExpect: 100-continue\r\n\r\n"),
            status: vec![100]
        },
        Case {
            name: "Header case insensitive",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nhoSt: bench\r\n\r\n"),
            status: vec![200]
        },
        Case {
            name: "Header tab",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost:\tbench\r\n\r\n"),
            status: vec![200]
        },
        Case {
            name: "Header empty value",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nEmpty:\r\n\r\n"),
            status: vec![200]
        },
        Case {
            name: "Header invalid characters",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nInvalid[]: something\r\n\r\n"),
            status: vec![400]
        },
        Case {
            name: "Header Host missing",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nOther: something\r\n\r\n"),
            status: vec![400]
        },
        Case {
            name: "Header multiple Host",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nHost: something\r\n\r\n"),
            status: vec![400]
        },
        Case {
            name: "Header negative content length",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nContent-Length: -1234\r\n\r\n"),
            status: vec![400]
        },
        Case {
            name: "Header overflowing negative content length",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nContent-Length: -123456789123456789123456789\r\n\r\n"),
            status: vec![400]
        },
        Case {
            name: "Header non numeric content length",
            request: Request::Raw("GET /index.html HTTP/1.1\r\nHost: bench\r\nContent-Length: something\r\n\r\n"),
            status: vec![400]
        },
        Case {
            name: "Invalid Prefix",
            request: Request::Raw("InvalidGET /index.html HTTP/1.1\r\nHost: bench\r\n\r\n"),
            status: vec![400]
        },
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
            name: "Valid POST",
            request: Request::Structured {
                method: "POST",
                path: "/",
                version: "HTTP/1.1",
                headers: vec!["Host: bench", "Content-Length: 5"],
                body: "hello",
            },
            status: vec![200, 404]
        },
    ]
}
