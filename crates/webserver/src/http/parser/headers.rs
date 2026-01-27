use crate::http::headers::Headers;
use crate::http::http_error::HttpError;
use crate::http::status_code::StatusCode;

pub fn parse_headers(raw_headers: String) -> Result<Headers, HttpError> {

    let mut headers = Headers::new();

    for raw_header in raw_headers.split("\r\n") {

        let header = raw_header.split_once(":");

        if header.is_none() {
            return Err(HttpError::new(StatusCode::BadRequest, format!("invalid header: {}", raw_header)));
        }

        let name = header.unwrap().0.trim().to_lowercase().to_string();
        let value = header.unwrap().1.trim().to_string();

        headers.add(name, value);
    }

    Ok(headers)
}


