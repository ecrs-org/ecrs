mod aco;
mod ga;
mod pso;
mod ff;

extern crate core;

use crate::aco::probe::CsvProbe;
use crate::aco::AntSystemCfg;
use crate::ga::{*};
use crate::pso::{*};
use ff::*;
use ff::auxiliary::*;
use ff::probe::console_probe::ConsoleProbe;


fn main() {
  ga_example();
  // ants_example_run();
  // pso_demo();
	// firefly_example();
}

fn firefly_example() {
  let mut alg = FireflyAlgorithm{
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: Box::new((ConsoleProbe{}))
  };

  alg.execute();
}

fn ants_example_run() {
  let (cities, cost) = aco::generate_tsp_cost(10);
  aco::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let probe = Box::new(CsvProbe::new());
  let heuristic = aco::create_heuristic_from_weights(&cost);

  let ant_s = aco::AntSystem::new(AntSystemCfg {
    weights: cost,
    heuristic,
    probe,
    ants_num: 100,
    ..AntSystemCfg::default()
  });

  ant_s.execute();
}

fn ga_example() {
  // let mut alg = GeneticAlgorithm::new(GeneticAlgorithmCfg {
  //   mutation_rate: 0.08,
  //   selection_rate: 0.5,
  //   generation_upper_bound: 200,
  //   population_size: 100,
  //   fitness_fn: rastrigin_fitness_function,
  //   mutation_operator: rastrigin_mutation_operator,
  //   population_factory: rastrigin_population_factory,
  //   eps: 1e-4,
  //   probe: Box::new(GAStdoutProbe{}),
  //   crossover_operator: rastrigin_crossover_operator
  // });

  // alg.run();

  GeneticAlgorithm::new(GeneticAlgorithmCfg {
    mutation_rate: 0.1,
    selection_rate: 0.5,
    generation_upper_bound: 200,
    population_size: 400,
    fitness_fn: quadratic_function,
    mutation_operator: quadratic_mutation_operator,
    population_factory: quadratic_population_factory,
    eps: 1e-4,
    // probe: Box::new(ga::StdoutProbe{}),
    // probe: Box::new(ga::CsvProbe::new("ga_testing.csv".to_owned())),
    probe: Box::new(ga::JsonProbe::new("ga_testing.json".to_owned())),
    crossover_operator: quadratic_crossover_operator,
  }).run();
}
