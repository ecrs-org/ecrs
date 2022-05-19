mod aco;

extern crate core;
use crate::aco::probe::CsvProbe;
use crate::aco::AntSystemCfg;

fn main() {
  ants_example_run();
}

fn ants_example_run() {
  let (cities, cost) = aco::generate_tsp_cost(60);
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
