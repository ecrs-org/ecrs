mod aco;
mod ga;
mod pso;
mod ff;
mod examples;

use ga::*;


fn main() {
  let mut ga_instance = ga::Builder::new()
    .set_max_generation_count(100)
    .set_mutation_rate(0.5f64)
    .build()
    .run();
}

