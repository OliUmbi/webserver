use crate::http::status_code::StatusCode;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Action {
    Fixed {
        root: PathBuf,
        fallback: Option<PathBuf>,
    },
    Proxy {
        location: String,
    },
    Redirect {
        location: String,
        status_code: StatusCode,
    },
}
