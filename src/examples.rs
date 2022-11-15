
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

pub fn pso_example() {
    let iterations = 2000;

    let console_probe = Box::new(pso::probe::console_probe::ConsoleProbe::new());
    let csv_probe = Box::new(pso::probe::csv_probe::CsvProbe::new("pso_example.csv"));
    let json_probe = Box::new(pso::probe::json_probe::JsonProbe::new("pso_example.json"));
    let probes : Vec<Box<dyn pso::probe::Probe>> = vec![console_probe, csv_probe, json_probe];
    let multi_probe = Box::new(pso::probe::multi_probe::MultiProbe::new(probes));
    let iteration_count_probe = Box::new(pso::probe::iteration_count_probe::IterationCountProbe::new(multi_probe, 50, iterations));
    // let timed_probe = Box::new(pso::probe::timed_probe::TimedProbe::new(multi_probe, 3, iterations));


    let mut algorithm = PSOAlgorithmBuilder::new()
        .set_dimensions(3)
        .set_iterations(iterations)
        .set_probe(iteration_count_probe)
        .build();

    algorithm.run();
}
