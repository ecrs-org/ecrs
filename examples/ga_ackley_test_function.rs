#![cfg(feature = "ga")]

use ecrs::{ga, test_functions};
mod util;

fn main() {
  let _ = util::init_logging();

  let best_individual = ga::Builder::with_rvc()
    .set_max_duration(std::time::Duration::from_secs(5))
    .fitness_fn(|x| -test_functions::ackley(x))
    .dim(4)
    .build()
    .run()
    .unwrap();

  println!("4D ackley function zero approximation {:#?}", best_individual)
}

#[cfg(not(feature = "ga"))]
compile_error!("Required feature \"ga\" is not enabled");
