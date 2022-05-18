extern crate core;

// mod genetic;
mod particle_swarm_optimization;

// use genetic::genetic_tsp::{GeneticAlgorithm, Config};
use particle_swarm_optimization::particle_swarm_optimization::pso_demo;



fn main() {
  // let alg = GeneticAlgorithm {
  //   config: Config {
  //     elite_size: 20,
  //     num_of_cites: 25,
  //     pop_size: 100,
  //     mutation_rate: 0.01,
  //     generations: 500,
  //   }
  // };
  // alg.run();

  pso_demo();
}
