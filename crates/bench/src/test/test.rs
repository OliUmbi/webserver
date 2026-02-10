use std::sync::{mpsc, Arc};
use crate::test::configuration::Configuration;
use crate::test::log::{Logger};

pub trait Test: Send + Sync {
    fn name(&self) -> String;

    fn run(&self, configuration: Arc<Configuration>, logger: Arc<Logger>);
}