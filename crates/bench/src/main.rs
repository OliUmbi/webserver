use crate::bench::Bench;
use crate::stress::stress::Stress;
use crate::test::test::{Test};

mod compliance;
mod stress;
mod test;
mod bench;
mod http;

fn main() {

    let mut tests: Vec<Box<dyn Test>> = Vec::new();
    tests.push(Box::new(Stress::new()));
    tests.push(Box::new(Stress::new()));
    tests.push(Box::new(Stress::new()));
    tests.push(Box::new(Stress::new()));

    let mut bench = Bench::new(tests);
    bench.start();
}
