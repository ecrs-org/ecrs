
use crate::aco::probe::CsvProbe;
use crate::aco::AntSystemCfg;
use crate::ga;
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
  let res = ga::Builder::new()
    .set_max_generation_count(100)
    .set_mutation_rate(0.5f64)
		.set_population_size(100)
		.set_fitness_fn(ga::example::quadratic_fn)
		.set_crossover_operator(ga::operators::crossover::single_point)
		.set_mutation_operator(ga::operators::mutation::range_compliment)
		.set_population_generator(ga::example::quadratic_population_factory)
		.set_eps(0.01)
		.set_probe(Box::new(ga::probe::stdout_probe::StdoutProbe{}))
    .build()
    .run();

	println!("{:?}", res);
}
