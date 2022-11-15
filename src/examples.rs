
use crate::aco::probe::CsvProbe;
use crate::aco::AntSystemCfg;
use crate::{ga, pso};
use crate::ff::*;
use crate::ff::auxiliary::*;
use crate::ff::probe::console_probe::ConsoleProbe;
use crate::pso::builder::PSOAlgorithmBuilder;

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
