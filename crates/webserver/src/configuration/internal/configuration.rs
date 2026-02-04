use crate::configuration::internal::route::Route;
use crate::configuration::internal::server::Server;

#[derive(Debug)]
pub struct Configuration {
    pub headless: bool,
    pub server: Server,
    pub routes: Vec<Route>,
}
