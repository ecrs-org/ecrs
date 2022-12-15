use ecrs::{ga, test_functions};

fn main() {
  let best_individual = ga::Builder::with_rvc()
    .fitness_fn(test_functions::ackley)
    .dim(4)
    .build()
    .run()
    .unwrap();

  println!("4D ackley function zero approximation {:#?}", best_individual)
}
