use std::sync::mpsc;
use crate::test::configuration::Configuration;
use crate::test::log::{Logger};

pub trait Test {
    fn name(&self) -> String;

    fn run(&self, configuration: Configuration, logger: Logger);
}