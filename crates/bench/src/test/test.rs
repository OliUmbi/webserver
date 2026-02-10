use crate::test::configuration::Configuration;
use crate::test::log::Logger;
use std::sync::Arc;

pub trait Test: Send + Sync {
    fn name(&self) -> String;

    fn run(&self, configuration: Arc<Configuration>, logger: Arc<Logger>);
}