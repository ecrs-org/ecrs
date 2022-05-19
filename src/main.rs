mod ga;

use crate::ga::{*};

fn main() {
  ga_example();
}

fn ga_example() {
  let mut alg = GeneticAlgorithm::new(GeneticAlgorithmCfg {
    mutation_rate: 0.08,
    selection_rate: 0.5,
    generation_upper_bound: 200,
    population_size: 100,
    fitness_fn: rastrigin_fitness_function,
    mutation_operator: rastrigin_mutation_operator,
    population_factory: rastrigin_population_factory,
    eps: 1e-4,
    probe: Box::new(GAStdoutProbe{}),
    crossover_operator: rastrigin_crossover_operator
  });

  // alg.run();

  let mut alg_quadratic = GeneticAlgorithm::new(GeneticAlgorithmCfg {
    mutation_rate: 0.1,
    selection_rate: 0.5,
    generation_upper_bound: 200,
    population_size: 400,
    fitness_fn: quadratic_function,
    mutation_operator: quadratic_mutation_operator,
    population_factory: quadratic_population_factory,
    eps: 1e-4,
    probe: Box::new(GAStdoutProbe{}),
    crossover_operator: quadratic_crossover_operator,
  });

  alg_quadratic.run();
}
