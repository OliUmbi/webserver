pub struct Configuration {
    threads: usize,
    port: usize,
    timeout: usize,
    max_header_length: usize,
    max_body_length: usize,
    routes: Vec<Route>,
}

pub struct Route {
    path: String,
    target: String,
}