use std::fmt::format;
use crate::configuration::internal::configuration::Configuration;
use crate::handler::handler_error::HandlerError;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use crate::parser;
use crate::server::connection::Connection;
use std::net::TcpStream;

pub fn handle(
    location: &String,
    request: &mut Request,
    connection: &mut Connection,
    configuration: &Configuration,
) -> Result<Response, HandlerError> {
    let upstream = TcpStream::connect(location).map_err(|_| HandlerError::new(StatusCode::ServiceUnavailable, "Failed to connect to upstream service"))?;

    let mut upstream_connection = Connection::new(upstream).map_err(|_| HandlerError::new(StatusCode::ServiceUnavailable, "Failed to connect to upstream service"))?;

    request.body.read(connection).map_err(|error| HandlerError::new(error.status, error.message))?;

    upstream_connection.write_request(request).map_err(|_| HandlerError::new(StatusCode::ServiceUnavailable, "Failed to forward to upstream service"))?;

    let upstream_response = parser::response::parse(&mut upstream_connection, &configuration).map_err(|_| HandlerError::new(StatusCode::BadGateway, "Server sent invalid response"))?;
    
    Ok(upstream_response)
}
