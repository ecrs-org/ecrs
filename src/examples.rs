
use crate::aco::probe::CsvProbe;
use crate::aco::AntSystemCfg;
use crate::ga::*;
use crate::pso::*;
use crate::ff::*;
use crate::ff::auxiliary::*;
use crate::ff::probe::console_probe::ConsoleProbe;

pub fn firefly_example() {
  let mut alg = FireflyAlgorithm{
    config: FireflyAlgorithmCfg::default(),
    brightness_function: rastrigin,
    probe: Box::new(ConsoleProbe{})
  };

  alg.execute();
}

pub fn ants_example_run() {
  let (cities, cost) = crate::aco::generate_tsp_cost(30);
  crate::aco::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let probe = Box::new(CsvProbe::new());
  let heuristic = crate::aco::create_heuristic_from_weights(&cost);

  let ant_s = crate::aco::AntSystem::new(AntSystemCfg {
    weights: cost,
    heuristic,
    probe,
    ants_num: 100,
    iteration: 1000,
    ..AntSystemCfg::default()
  });

  ant_s.execute();
}

pub fn ga_example() {
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
    probe: Box::new(crate::ga::JsonProbe::new("ga_testing.json".to_owned())),
    crossover_operator: quadratic_crossover_operator,
  }).run();
}
