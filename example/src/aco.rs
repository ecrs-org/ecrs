use ecrs::aco::{probe::CsvProbe, AntSystemCfg};


pub fn ants_example_run() {
  let (cities, cost) = ecrs::aco::generate_tsp_cost(30);
  ecrs::aco::write_cities_csv(&cities, "cities.csv").expect("Error while writing city file");

  let probe = Box::new(CsvProbe::new());
  let heuristic = ecrs::aco::create_heuristic_from_weights(&cost);

  let ant_s = ecrs::aco::AntSystem::new(AntSystemCfg {
    weights: cost,
    heuristic,
    probe,
    ants_num: 100,
    iteration: 1000,
    ..AntSystemCfg::default()
  });

  ant_s.execute();
}
