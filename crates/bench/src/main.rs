use std::sync::Arc;
use crate::bench::Bench;
use crate::compliance::compliance::Compliance;
use crate::stress::stress::Stress;
use crate::test::test::{Test};

mod compliance;
mod stress;
mod test;
mod bench;
mod http;

fn main() {

    let mut tests: Vec<Arc<dyn Test>> = Vec::new();
    tests.push(Arc::new(Stress::new()));
    tests.push(Arc::new(Compliance::new()));

    let mut bench = Bench::new(tests);
    bench.start();
}
